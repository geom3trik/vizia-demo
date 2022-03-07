use vizia::*;

pub mod app_data;
pub use app_data::*;

fn main() {
    let window_description = WindowDescription::new().with_title("Todos Application");
    let app = Application::new(window_description, |cx|{

        #[cfg(debug_assertions)]
        cx.add_stylesheet("src/style.css");
        #[cfg(not(debug_assertions))]
        cx.add_theme(include_str!("style.css"));

        AppData {
            todo_items: vec![
                TodoItem {
                    done: false,
                    text: "Finish vizia demo".to_string(),
                },

                TodoItem {
                    done: false,
                    text: "Finish vizia book".to_string(),
                },
            ],
            new_item: "".to_string(),
        }.build(cx);

        HStack::new(cx, |cx|{
            Textbox::new(cx, AppData::new_item)
                .on_edit(|cx, text| cx.emit(AppEvent::NewItem(text.clone())));
            Button::new(cx, |cx| cx.emit(AppEvent::AddItem), |cx|{
                Label::new(cx, "Add")
            }).class("add");
        })
        .height(Auto)
        .child_space(Pixels(10.0))
        .col_between(Pixels(10.0));

        List::new(cx, AppData::todo_items, |cx, index, item|{
            VStack::new(cx, |cx|{
               HStack::new(cx, |cx|{
                    Checkbox::new(cx, item.then(TodoItem::done))
                        .on_toggle(move |cx| cx.emit(AppEvent::ToggleDone(index)));
                    Label::new(cx, item.then(TodoItem::text));
               });
            })
            .class("item");
        });

    });

    app.run();
}