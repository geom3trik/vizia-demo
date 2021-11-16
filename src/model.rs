use std::{fs::File, io::{BufReader, Read, Seek, SeekFrom}};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use vizia::*;

#[derive(Debug)]
pub enum AppEvent {

    Test,
    

    Selection(usize),
    SelectionLeft,
    SelectionRight,
    SelectionUp,
    SelectionDown,
    SelectionStart,
    SelectionEnd,
    SelectionPageUp,
    SelectionPageDown,

    SetCursor(usize),

    SelectionDrag,
    SelectionDrop,

    // Settings
    SetLittleEndian,
    SetBigEndian,
    SetColumns(usize),

}


#[derive(Lens)]
pub struct HexData {
    pub reader: BufReader<File>,
    pub view: Vec<(usize, u8)>,
    pub view_start: usize,
    pub seek_pos: usize,
    pub view_changed: bool,

    pub endian: Endian,

    pub selection: Selection,
    pub inspector: Inspector,

}

impl HexData {
    pub fn new() -> std::io::Result<Self> {
        let file = File::open("Moon.jpg")?;
        let file_length = file.metadata().expect("Failed to get file metadata").len() as usize;
        let mut reader = BufReader::new(file);

        let mut view = vec![0u8; 8 * 16];
        let num = reader.read(&mut view).expect("Failed to read data");

        let current_pos  = reader.seek(SeekFrom::Current(0)).expect("Failed to get current position");

        Ok(Self {
            reader,
            view: view.into_iter().enumerate().collect(),
            seek_pos: current_pos as usize,
            view_start: 0,
            view_changed: false,

            endian: Endian::LittleEndian,

            selection: Selection {
                data_len: file_length,
                columns: 8,
                cursor: 0,
                anchor: 0,
                dragging: false,
            },

            inspector: Inspector::default(),
        })
    }

    pub fn next_line(&mut self) {
        self.view_start += 8;
        self.read_view();
    }

    pub fn prev_line(&mut self) {
        self.view_start -= 8;
        self.read_view();
    }
    
    pub fn read_view(&mut self) {
        self.reader.seek(SeekFrom::Start(self.view_start as u64)).expect("Failed to seek");
        let mut view = vec![0u8; 8 * 16];
        self.reader.read(&mut view).expect("Failed to read data");

        self.view = (self.view_start..self.view_start+128).into_iter().zip(view.into_iter()).collect();
    }

    pub fn calc_inspector(&mut self) {
        self.reader.seek(SeekFrom::Start(self.selection.anchor as u64)).expect("Failed to seek");
        let mut view = vec![0u8; 8];
        self.reader.read(&mut view).expect("Failed to read data");

        // TODO - This will fail if at the end of the file
        if self.endian == Endian::LittleEndian {
            self.inspector = Inspector {
                num_i8: view[0] as i8,
                num_u8: view[0] as u8,
                num_i16: (&view[0..2]).read_i16::<LittleEndian>().expect("Failed to read i16"),
                num_u16: (&view[0..2]).read_u16::<LittleEndian>().expect("Failed to read u16"),
                num_i32: (&view[0..4]).read_i32::<LittleEndian>().expect("Failed to read i32"),
                num_u32: (&view[0..4]).read_u32::<LittleEndian>().expect("Failed to read u32"),
            }
        } else {
            self.inspector = Inspector {
                num_i8: view[0] as i8,
                num_u8: view[0] as u8,
                num_i16: (&view[0..2]).read_i16::<BigEndian>().expect("Failed to read i16"),
                num_u16: (&view[0..2]).read_u16::<BigEndian>().expect("Failed to read u16"),
                num_i32: (&view[0..4]).read_i32::<BigEndian>().expect("Failed to read i32"),
                num_u32: (&view[0..4]).read_u32::<BigEndian>().expect("Failed to read u32"),
            }
        }
    }
}

impl Model for HexData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Test => {
                    self.next_line();
                    return true;
                }

                AppEvent::Selection(index) => {
                    self.selection.anchor = *index;
                    self.selection.cursor = *index;

                    self.calc_inspector();

                    return true;
                }

                AppEvent::SetCursor(index) => {
                    self.selection.cursor = *index;
                    return true;
                }

                AppEvent::SelectionRight => {
                    self.selection.cursor = self.selection.cursor.saturating_add(1);
                    if self.selection.cursor >= self.selection.data_len {
                        self.selection.cursor = self.selection.data_len - 1;
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.selection.anchor = self.selection.cursor;
                    }

                    if self.selection.cursor >= self.view_start + 128 {
                        self.next_line();
                    }

                    self.calc_inspector();

                    return true;
                }

                AppEvent::SelectionLeft => {
                    
                    self.selection.cursor = self.selection.cursor.saturating_sub(1);
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.selection.anchor = self.selection.cursor;
                    }

                    if self.selection.cursor < self.view_start {
                        self.prev_line();
                    }

                    self.calc_inspector();
                    
                    return true;
                }

                AppEvent::SelectionUp => {
                    if self.selection.cursor >= self.selection.columns {
                        self.selection.cursor = self.selection.cursor.saturating_sub(self.selection.columns);
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.selection.anchor = self.selection.cursor;
                    }

                    if self.selection.cursor < self.view_start {
                        self.prev_line();
                    }

                    self.calc_inspector();

                    return true;
                }

                AppEvent::SelectionDown => {
                    if self.selection.cursor + self.selection.columns < self.selection.data_len {
                        self.selection.cursor = self.selection.cursor + self.selection.columns;
                    }
                    if !cx.modifiers.contains(Modifiers::SHIFT) {
                        self.selection.anchor = self.selection.cursor;
                    }

                    if self.selection.cursor >= self.view_start + 128 {
                        self.next_line();
                    }

                    self.calc_inspector();

                    return true;
                }

                // TODO
                // AppEvent::SelectionStart => {
                //     self.selection.cursor = 0;
                //     self.selection.anchor = 0;

                // }

                AppEvent::SelectionDrag => {
                    self.selection.dragging = true;
                    return true;
                }

                AppEvent::SelectionDrop => {
                    self.selection.dragging = false;
                    return true;
                }

                AppEvent::SetColumns(columns) => {
                    self.selection.columns = *columns;
                    return true;
                }

                AppEvent::SetLittleEndian => {
                    self.endian = Endian::LittleEndian;
                    self.calc_inspector();
                    return true;
                }

                AppEvent::SetBigEndian => {
                    self.endian = Endian::BigEndian;
                    self.calc_inspector();
                    return true;
                }

                _=> {}
            }
        }

        return false;
    }
}

// Selection is separate from `HexData` or else updating the selection would rebuild the grids
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

#[derive(Debug, Default)]
pub struct Inspector {
    pub num_i8: i8,
    pub num_u8: u8,
    pub num_i16: i16,
    pub num_u16: u16,
    pub num_i32: i32,
    pub num_u32: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    BigEndian,
    LittleEndian,
}