use std::{fs::File, io::{BufReader, Read}};

use vizia::*;

#[derive(Debug)]
pub enum AppEvent {
    Selection(usize),
    SelectionLeft,
    SelectionRight,
    SelectionUp,
    SelectionDown,

    SetCursor(usize),

    SelectionDrag,
    SelectionDrop,

    SetLittleEndian,
    SetBigEndian,
}


#[derive(Lens)]
pub struct HexData {
    pub reader: BufReader<File>,
    pub view: Vec<u8>,

}

impl HexData {
    pub fn new() -> std::io::Result<Self> {
        let file = File::open("Moon.jpg")?;
        let mut reader = BufReader::new(file);

        let mut view = vec![0u8; 8 * 16];
        let num = reader.read(&mut view).expect("Failed to read data");

        println!("Bytes read: {}", num);

        println!("{:?}", view);
        
        Ok(Self {
            reader,
            view,
        })
    }
}

impl Model for HexData {

}

// Selection is separate from `HexData` or else updating the selection would rebuild the grids
#[derive(Lens)]
pub struct Selection {
    pub data_len: usize,
    pub columns: usize,
    pub anchor: usize,
    pub cursor: usize,
    pub dragging: bool,
}

impl Selection {
    pub fn start(&self) -> usize {
        self.cursor.min(self.anchor)
    }

    pub fn end(&self) -> usize {
        self.cursor.max(self.anchor)
    }
}

impl Model for Selection {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(app_event) = event.message.downcast() {
            match app_event {

                AppEvent::Selection(index) => {
                    self.anchor = *index;
                    self.cursor = *index;
                    return true;
                }

                AppEvent::SetCursor(index) => {
                    self.cursor = *index;
                    return true;
                }

                AppEvent::SelectionRight => {
                    self.cursor = self.cursor.saturating_add(1);
                    if self.cursor >= self.data_len {
                        self.cursor = self.data_len - 1;
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.anchor = self.cursor;
                    }
                    return true;
                }

                AppEvent::SelectionLeft => {
                    
                    self.cursor = self.cursor.saturating_sub(1);
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.anchor = self.cursor;
                    }
                    
                    return true;
                }

                AppEvent::SelectionUp => {
                    if self.cursor >= self.columns {
                        self.cursor = self.cursor.saturating_sub(self.columns);
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.anchor = self.cursor;
                    }
                    return true;
                }

                AppEvent::SelectionDown => {
                    if self.cursor + self.columns < self.data_len {
                        self.cursor = self.cursor + self.columns;
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.anchor = self.cursor;
                    }

                    return true;
                }

                AppEvent::SelectionDrag => {
                    self.dragging = true;
                    return true;
                }

                AppEvent::SelectionDrop => {
                    self.dragging = false;
                    return true;
                }

                _=> {}
            }
        }

        return false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}

#[derive(Lens)]
pub struct Settings {
    pub endian: Endian,
}

impl Model for Settings {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetLittleEndian => {
                    self.endian = Endian::LittleEndian;
                    return true;
                }

                AppEvent::SetBigEndian => {
                    self.endian = Endian::BigEndian;
                    return true;
                }

                _=> {}
            }
        }

        return false;
    }
}