use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    pub todo_items: Vec<TodoItem>,
    pub new_item: String,
}

#[derive(Clone, Data, Lens)]
pub struct TodoItem {
    pub done: bool,
    pub text: String,
}

pub enum AppEvent {
    NewItem(String),
    AddItem,
    DeleteItem(usize),
    ToggleDone(usize),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::NewItem(text) => {
                self.new_item = text.clone();
            }

            AppEvent::AddItem => {
                if !self.new_item.is_empty() {
                    self.todo_items.push(TodoItem {
                        done: false,
                        text: self.new_item.clone(),
                    })
                }
            }

            AppEvent::DeleteItem(index) => {
                self.todo_items.remove(*index);
            }

            AppEvent::ToggleDone(index) => {
                self.todo_items[*index].done ^= true;
            }
        });
    }
}
