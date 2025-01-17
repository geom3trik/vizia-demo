use vizia::prelude::*;

pub mod app_data;
pub use app_data::*;

fn main() {
    Application::new(|cx| {
        #[cfg(debug_assertions)]
        cx.add_stylesheet("src/style.css");
        #[cfg(not(debug_assertions))]
        cx.add_theme(include_str!("style.css"));

        AppData {
            todo_items: vec![
                TodoItem {
                    done: false,
                    text: "Finish vizia demo".to_owned(),
                },
                TodoItem {
                    done: false,
                    text: "Finish vizia book".to_owned(),
                },
                TodoItem {
                    done: false,
                    text: "Think of a better demo".to_owned(),
                },
            ],
            new_item: "".to_string(),
        }
        .build(cx);

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::new_item)
                .on_edit(|cx, text| cx.emit(AppEvent::NewItem(text.clone())));
            Button::new(
                cx,
                |cx| cx.emit(AppEvent::AddItem),
                |cx| Label::new(cx, "Add"),
            )
            .cursor(CursorIcon::Hand)
            .class("add");
        })
        .height(Auto)
        .child_space(Pixels(10.0))
        .col_between(Pixels(10.0));

        List::new(cx, AppData::todo_items, |cx, index, item| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Checkbox::new(cx, item.then(TodoItem::done))
                        .on_toggle(move |cx| cx.emit(AppEvent::ToggleDone(index)));
                    Label::new(cx, item.then(TodoItem::text));
                });
            })
            .class("item");
        });
    })
    // .background_color(Color::from("#f1f1f1"))
    .title("Todos Application")
    .canvas("win")
    .inner_size((600, 400))
    .run();
}
