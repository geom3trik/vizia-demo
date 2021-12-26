use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use vizia::*;

mod model;
use model::{AppEvent, Endian, HexData};

mod grid;
use grid::Grid;

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx|{

        HexData::new().expect("Failed to load file").build(cx);

        // TODO - This doesn't work yet due to a problem with layout
        // HStack::new(cx, |cx|{
        //     Button::new(cx, |cx| cx.emit(AppEvent::SetColumns(4)), |cx|{
        //         Label::new(cx, "4");
        //     });

        //     Button::new(cx, |cx| cx.emit(AppEvent::SetColumns(8)), |cx|{
        //         Label::new(cx, "8");
        //     });

        //     Button::new(cx, |cx| cx.emit(AppEvent::SetColumns(16)), |cx|{
        //         Label::new(cx, "16");
        //     });
        // }).height(Pixels(30.0));

        //Button::new(cx, |cx| cx.emit(AppEvent::Test), |cx| {});

        KeyView::new(cx, |cx|{
            VStack::new(cx, |cx|{
                HStack::new(cx, |cx|{

                    // Offset Labels
                    VStack::new(cx, |cx|{
                        Binding::new(cx, HexData::view_start, |cx, data|{
                            Label::new(cx, "Offset").width(Pixels(80.0)).font_size(18.0);
                            ForEach::new(cx, 0..16, move |cx, index|{
                                let view_start = *data.get(cx);
                                Label::new(cx, &format!("{:08X}", view_start + index * 8)).width(Stretch(1.0));
                            });
                        });
                    }).width(Pixels(30.0)).top(Pixels(30.0));

                    // Hex Grid
                    VStack::new(cx, |cx|{
                        Label::new(cx, "Hex Grid").font_size(18.0);
                        //Binding::new(cx, Settings::columns, |cx, columns|{
                            //let columns = *columns.get(cx); 
               
                            Grid::new(cx, 8, 16, HexData::view, |cx, columns, item|{
                                Binding::new(cx, HexData::selection, move |cx, selection|{
                                    let item_index = item.value(cx).0;
                                    let select_start = selection.get(cx).start();
                                    let select_end = selection.get(cx).end();
                                    let is_dragging = selection.get(cx).dragging;
                                    //println!("Item Index: {} {}", select_start, select_end);
                                    //println!("index {} start {} end: {}", item_index, select_start, select_end);
                                    Label::new(cx, &format!("{:02X}", item.value(cx).1))
                                    //Label::new(cx, &format!("{}", item.index()))
                                        .background_color(
                                            if item_index >= select_start && item_index <= select_end {
                                                Color::rgb(153, 217, 234)
                                            } else {
                                                Color::white()
                                            }
                                        )
                                        .size(Stretch(1.0))
                                        .child_space(Stretch(1.0))
                                        .on_press(cx, move |cx| {
                                            println!("{}", item_index);
                                            cx.emit(AppEvent::Selection(item_index));
                                            cx.emit(AppEvent::SelectionDrag);
                                        })
                                        .on_release(cx, move |cx| {
                                            cx.emit(AppEvent::SelectionDrop);
                                        })
                                        .on_hover(cx, move |cx| {
                                            if is_dragging {
                                                cx.emit(AppEvent::SetCursor(item_index));
                                            }
                                        });
                                });
                            }).width(Pixels(30.0 * 8.0));
                        //});
                    });
        
                    // TODO - Factor into custom view to reduce duplication
                    // Text Grid
                    VStack::new(cx, |cx|{
                        Label::new(cx, "Decoded Text").font_size(18.0);
                        Grid::new(cx, 8, 16, HexData::view, |cx, columns, item|{
                            Binding::new(cx, HexData::selection, move |cx, selection|{
                                let item_index = item.value(cx).0;
                                let select_start = selection.get(cx).start();
                                let select_end = selection.get(cx).end();
                                let is_dragging = selection.get(cx).dragging;
                                Label::new(cx, &if item.value(cx).1 <= 32 {".".to_string()} else {format!("{}", item.value(cx).1 as char)})
                                    .background_color(
                                        if item_index >= select_start && item_index <= select_end {
                                            Color::rgb(153, 217, 234)
                                        } else {
                                            Color::white()
                                        }
                                    )
                                    .size(Stretch(1.0))
                                    .child_space(Stretch(1.0))
                                    .on_press(cx, move |cx| {
                                        cx.emit(AppEvent::Selection(item_index));
                                        cx.emit(AppEvent::SelectionDrag);
                                    })
                                    .on_release(cx, move |cx| {
                                        cx.emit(AppEvent::SelectionDrop);
                                    })
                                    .on_hover(cx, move |cx| {
                                        if is_dragging {
                                            cx.emit(AppEvent::SetCursor(item_index));
                                        }
                                    });
                            });
                        }).width(Pixels(30.0 * 8.0));
                    });
                    
                    // Inspector
                    VStack::new(cx, |cx|{
                        Label::new(cx, "Inspector").font_size(18.0);
                        Binding::new(cx, HexData::inspector, |cx, inspector|{
                                    
                            let num_i8 = inspector.get(cx).num_i8;
                            let num_u8 = inspector.get(cx).num_u8;
                            let num_i16 = inspector.get(cx).num_i16;
                            let num_u16 = inspector.get(cx).num_u16;
                            let num_i32 = inspector.get(cx).num_i32;
                            let num_u32 = inspector.get(cx).num_u32;

                            VStack::new(cx, move |cx|{
                            
                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "i8").width(Pixels(50.0));
                                    Label::new(cx, &num_i8.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black());
                                }).height(Pixels(30.0));

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "u8").width(Pixels(50.0));
                                    Label::new(cx, &num_u8.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black());
                                }).height(Pixels(30.0));

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "i16").width(Pixels(50.0));
                                    Label::new(cx, &num_i16.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black());
                                }).height(Pixels(30.0));

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "u16").width(Pixels(50.0));
                                    Label::new(cx, &num_u16.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black());
                                }).height(Pixels(30.0));

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "i32").width(Pixels(50.0));
                                    Label::new(cx, &num_i32.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black()); 
                                }).height(Pixels(30.0));

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "u32").width(Pixels(50.0));
                                    Label::new(cx, &num_u32.to_string())
                                        .width(Stretch(1.0)).border_width(Pixels(1.0)).border_color(Color::black());
                                }).height(Pixels(30.0));

                                Binding::new(cx, HexData::endian, |cx, endian|{
                                    HStack::new(cx, move |cx|{
                                        Checkbox::new(cx, *endian.get(cx) == Endian::LittleEndian)
                                            .on_checked(cx, |cx| cx.emit(AppEvent::SetLittleEndian))
                                            .on_unchecked(cx, |cx| cx.emit(AppEvent::SetLittleEndian));
                                        Label::new(cx, "Little Endian");
                                        Checkbox::new(cx, *endian.get(cx) == Endian::BigEndian)
                                            .on_checked(cx, |cx| cx.emit(AppEvent::SetBigEndian))
                                            .on_unchecked(cx, |cx| cx.emit(AppEvent::SetBigEndian));
                                        Label::new(cx, "Big Endian");
                                    }).child_top(Stretch(1.0)).child_bottom(Stretch(1.0)).height(Pixels(30.0));
                                });

                            }).row_between(Pixels(5.0));
                        });
                    });
    
                }).child_space(Pixels(30.0)).col_between(Pixels(30.0));

                Element::new(cx).height(Pixels(1.0)).background_color(Color::black());

                // Footer
                Binding::new(cx, HexData::selection, |cx, selection|{
                    
                    let select_start = selection.get(cx).start();
                    let select_end = selection.get(cx).end();

                    HStack::new(cx, move |cx|{
                        Label::new(cx, &format!("Offset: {}", select_start));
                        if select_start != select_end {
                            Label::new(cx, &format!("Length: {}", select_end + 1 - select_start));
                        } else {
                            Label::new(cx, "");
                        }
                    }).height(Pixels(30.0)).background_color(Color::white());
                });
            });


        });
    }).run();
}

// A view to add keyboard shortcuts
// TODO - make this more general and not specific to thsi app
pub struct KeyView {
    builder: Option<Box<dyn Fn(&mut Context)>>,
}

impl KeyView {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self> 
    where F: 'static + Fn(&mut Context),
    {
        let handle = Self {
            builder: Some(Box::new(builder)),
        }.build(cx);

        cx.focused = handle.entity;

        handle
    }
}

impl View for KeyView {

    fn body(&mut self, cx: &mut Context) {
        if let Some(builder) = self.builder.take() {
            (builder)(cx);
            self.builder = Some(builder);
        }
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, key) => {
                    match *code {

                        Code::ArrowLeft => {
                            cx.emit(AppEvent::SelectionLeft);
                        }

                        Code::ArrowRight => {
                            cx.emit(AppEvent::SelectionRight);
                        }

                        Code::ArrowDown => {
                            cx.emit(AppEvent::SelectionDown);
                        }

                        Code::ArrowUp => {
                            cx.emit(AppEvent::SelectionUp);
                        }

                        Code::PageDown => {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                cx.emit(AppEvent::SelectionEnd);
                            } else {
                                cx.emit(AppEvent::SelectionPageDown);
                            }
                        }

                        Code::PageUp => {
                            if cx.modifiers.contains(Modifiers::CTRL) {
                                cx.emit(AppEvent::SelectionStart);
                            } else {
                                cx.emit(AppEvent::SelectionPageUp);
                            }
                        }

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}