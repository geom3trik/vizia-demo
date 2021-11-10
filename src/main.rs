use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use vizia::*;

mod model;
use model::{AppEvent, Endian, HexData, Selection, Settings};

mod grid;
use grid::Grid;

fn main() {
    Application::new(|cx|{

        HexData::new().expect("Failed to load file").build(cx);

        Selection {
            data_len: 8 * 16,
            columns: 8,
            cursor: 0,
            anchor: 0,
            dragging: false,
        }.build(cx);

        Settings {
            endian: Endian::LittleEndian,
        }.build(cx);

        KeyView::new(cx, |cx|{
            HStack::new(cx, |cx|{
                Grid::new(cx, 8, 16, HexData::view, |cx, columns, item|{
                    Binding::new(cx, Selection::root, move |cx, selection|{
                        let item_index = item.index();
                        let select_start = selection.get(cx).start();
                        let select_end = selection.get(cx).end();
                        let is_dragging = selection.get(cx).dragging;
                        Label::new(cx, &format!("{:02X}", item.value(cx)))
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
                }).width(Pixels(40.0 * 8.0));
    
                // TODO - Factor into custom view to reduce duplication
                Grid::new(cx, 8, 32, HexData::view, |cx, columns, item|{
                    Binding::new(cx, Selection::root, move |cx, selection|{
                        let item_index = item.index();
                        let select_start = selection.get(cx).start();
                        let select_end = selection.get(cx).end();
                        let is_dragging = selection.get(cx).dragging;
                        Label::new(cx, &if *item.value(cx) <= 32 {".".to_string()} else {format!("{}", *item.value(cx) as char)})
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
                }).width(Pixels(40.0 * 8.0));

                Binding::new(cx, HexData::view, |cx, data|{
                    Binding::new(cx, Selection::root, move |cx, selection|{
                        Binding::new(cx, Settings::endian, move |cx, endian|{

                            
                            let select_start = selection.get(cx).start();
                            let select_end = selection.get(cx).end();
                            let endian = endian.get(cx);

                            let num = &data.get(cx)[select_start..select_end+1];
                            let num_u8 = num[0] as u8;
                            let num_i8 = num[0] as i8;

                            let num_i16 = if select_end - select_start >= 1 {
                                match *endian {
                                    Endian::BigEndian => {
                                        (&num[0..2]).read_i16::<BigEndian>().unwrap().to_string()
                                    }

                                    Endian::LittleEndian => {
                                        (&num[0..2]).read_i16::<LittleEndian>().unwrap().to_string()
                                    }
                                }
                            } else {
                                "-".to_string()
                            };
                            
                            
                            VStack::new(cx, move |cx|{
                            
                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "i8").width(Pixels(50.0));
                                    Element::new(cx).width(Pixels(1.0)).background_color(Color::black());
                                    if let Some(num) = data.get(cx).get(select_start) {
                                        Label::new(cx, &num_i8.to_string()); 
                                    }
                                }).height(Pixels(30.0)).border_width(Pixels(1.0)).border_color(Color::black());

                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "u8").width(Pixels(50.0));
                                    Element::new(cx).width(Pixels(1.0)).background_color(Color::black());
                                    if let Some(num) = data.get(cx).get(select_start) {
                                        Label::new(cx, &num_u8.to_string()); 
                                    }
                                }).height(Pixels(30.0)).border_width(Pixels(1.0)).border_color(Color::black());

                                let num_i16 = num_i16.clone();
                                HStack::new(cx, move |cx|{
                                    Label::new(cx, "i16").width(Pixels(50.0));
                                    Element::new(cx).width(Pixels(1.0)).background_color(Color::black());
                                    Label::new(cx, &num_i16); 
                                }).height(Pixels(30.0)).border_width(Pixels(1.0)).border_color(Color::black());

                                // HStack::new(cx, move |cx|{
                                //     Label::new(cx, "u16").width(Pixels(50.0));
                                //     Element::new(cx).width(Pixels(1.0)).background_color(Color::black());
                                //     if select_end - select_start >= 1 {
                                //         let num = &data.get(cx)[select_start..select_end+1];
                                //         let num_i16 = (&num[0..2]).read_u16::<BigEndian>().expect("Failed to read u16");
                                //         Label::new(cx, &num_i16.to_string()); 
                                //     } else {
                                //         Label::new(cx, "-"); 
                                //     }
                                    
                                // }).height(Pixels(30.0)).border_width(Pixels(1.0)).border_color(Color::black());

                                // HStack::new(cx, move |cx|{
                                //     Label::new(cx, "i32").width(Pixels(50.0));
                                //     Element::new(cx).width(Pixels(1.0)).background_color(Color::black());
                                //     if select_end - select_start >= 3 {
                                //         let num = &data.get(cx)[select_start..select_end+1];
                                //         let num_i16 = (&num[0..4]).read_u16::<BigEndian>().expect("Failed to read u16");
                                //         Label::new(cx, &num_i16.to_string()); 
                                //     } else {
                                //         Label::new(cx, "-"); 
                                //     }
                                    
                                // }).height(Pixels(30.0)).border_width(Pixels(1.0)).border_color(Color::black());

                                Binding::new(cx, Settings::endian, |cx, endian|{
                                    HStack::new(cx, move |cx|{
                                        Checkbox::new(cx, *endian.get(cx) == Endian::LittleEndian)
                                            .on_checked(cx, |cx| cx.emit(AppEvent::SetLittleEndian))
                                            .on_unchecked(cx, |cx| cx.emit(AppEvent::SetLittleEndian));
                                        Label::new(cx, "Little Endian");
                                        Checkbox::new(cx, *endian.get(cx) == Endian::BigEndian)
                                            .on_checked(cx, |cx| cx.emit(AppEvent::SetBigEndian))
                                            .on_unchecked(cx, |cx| cx.emit(AppEvent::SetBigEndian));
                                        Label::new(cx, "Big Endian");
                                    }).child_top(Stretch(1.0)).child_bottom(Stretch(1.0));
                                }).height(Pixels(30.0));

                            }).row_between(Pixels(5.0));
                        });
                    });
                });

            }).child_space(Pixels(30.0)).col_between(Pixels(30.0));


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

                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}