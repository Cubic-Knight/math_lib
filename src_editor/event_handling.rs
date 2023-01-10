use std::collections::HashMap;
use crate::events::Event;
use crate::graphics::{
    get_menu, MenuGraphics, MenuLine,
    get_file, FileGraphics,
    display_menu, display_file
};
use crate::library_data::{
    Reference, LibraryData
};
use termwiz::{
    input::{KeyCode, Modifiers}
};

pub enum EditorState {
    InMenu,
    EditingFile,
    ShouldExit
}

pub struct EditorData {
    pub state: EditorState,
    pub menu: MenuGraphics,
    pub file: FileGraphics,
    pub dimensions: (usize, usize),
    pub indent: usize,
    pub lib_data: LibraryData,
    pub references: HashMap<String, Reference>
}

impl Default for EditorData {
    fn default() -> Self {
        EditorData {
            state: EditorState::InMenu,
            menu: get_menu().unwrap(),
            file: FileGraphics::default(),
            dimensions: (80, 24),
            indent: 4,
            lib_data: LibraryData {
                syntaxes: Vec::new(),
                definitions: Vec::new(),
                axioms: Vec::new(),
                theorems: Vec::new()
            },
            references: HashMap::new()
        }
    }
}

pub fn handle_event_in_menu(event: Event, editor_data: &mut EditorData) {
    match event {
        Event::KeyboardInterrupt => editor_data.state = EditorState::ShouldExit,
        Event::KeyPressedArrow((dy, 0), Modifiers::NONE) => {
            editor_data.menu.cursor = editor_data.menu.cursor
                .saturating_add_signed(dy)
                .clamp(2, editor_data.menu.lines.len());
            display_menu(&editor_data.menu, &editor_data.dimensions);
        },
        Event::KeyPressedArrow((dy, 0), Modifiers::SHIFT) => {
            editor_data.menu.camera = editor_data.menu.camera
                .saturating_add_signed(dy)
                .clamp(1, editor_data.menu.lines.len());
            display_menu(&editor_data.menu, &editor_data.dimensions);
        },
        Event::KeyPressedOther(KeyCode::Enter, Modifiers::NONE) => {
            let index = editor_data.menu.cursor - 1;
            if let Some(MenuLine::File(_, path, _, _)) = editor_data.menu.lines.get(index) {
                editor_data.file = get_file(
                    path.to_owned(), &editor_data.lib_data, &editor_data.references
                ).unwrap();
                editor_data.state = EditorState::EditingFile;
                display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
            };
        },
        Event::WindowResize(cols, rows) => {
            editor_data.dimensions = (cols, rows);
            display_menu(&editor_data.menu, &editor_data.dimensions);
        },
        _ => ()
    };
}

fn get_len_of_file_line(editor_data: &mut EditorData, index: usize) -> usize {
    editor_data.file.lines.get(index)
        .map(|line| line.chars.len() + 1)
        .unwrap_or(1)
}

pub fn handle_event_in_file_edition(event: Event, editor_data: &mut EditorData) {
    match event {
        Event::KeyPressedOther(KeyCode::Escape, Modifiers::NONE) => {
            editor_data.state = EditorState::InMenu;
            display_menu(&editor_data.menu, &editor_data.dimensions);
        },
        Event::KeyPressedArrow((0, dx), Modifiers::NONE) => {
            let (cy, cx) = editor_data.file.cursor;
            let line_len = get_len_of_file_line(editor_data, cy-1);
            if (cy, cx) == (1, 1) {
                if dx < 0 { return; }
            };
            if (cy, cx) == (editor_data.file.lines.len(), line_len) {
                if dx > 0 { return; }
            };
            editor_data.file.cursor = match cx.saturating_add_signed(dx) {
                0 => (cy-1, get_len_of_file_line(editor_data, cy-2)),
                x if x > line_len => (cy+1, 1),
                x => (cy, x)
            };
            display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
        },
        Event::KeyPressedArrow((dy, 0), Modifiers::NONE) => {
            let (cy, cx) = editor_data.file.cursor;
            let line_len = get_len_of_file_line(editor_data, cy-1);
            if cy == 1 && dy == -1 {
                if cx == 1 { return; };
                editor_data.file.cursor = (1, 1);
                display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
                return;
            };
            if cy == editor_data.file.lines.len() && dy == 1 {
                if cx == line_len { return; }
                editor_data.file.cursor = (editor_data.file.lines.len(), line_len);
                display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
                return;
            };
            let line_period = editor_data.dimensions.0 - editor_data.indent - 1;
            let line_height = line_len.saturating_sub(editor_data.indent + 1)
                .div_euclid(line_period);
            let cursor_height = cx.saturating_sub(editor_data.indent + 1)
                .div_euclid(line_period);
            match dy {
                -1 => match cx {
                    _ if cursor_height == 0 => {
                        let prev_line_len = get_len_of_file_line(editor_data, cy-2);
                        let prev_line_height = prev_line_len
                            .saturating_sub(editor_data.indent + 1)
                            .div_euclid(line_period);
                        let x = if prev_line_height == 0 {
                            cx
                        } else {
                            cx.max(editor_data.indent + 1) + prev_line_height * line_period
                        };
                        editor_data.file.cursor = (cy-1, x.min(prev_line_len))
                    },
                    _ => editor_data.file.cursor.1 -= line_period
                },
                1 => match cx {
                    _ if line_height == 0 => {
                        let next_line_len = get_len_of_file_line(editor_data, cy);
                        editor_data.file.cursor = (cy+1, cx.min(next_line_len));
                    },
                    _ if cursor_height == line_height => {
                        let next_line_len = get_len_of_file_line(editor_data, cy);
                        let x = cx - cursor_height * line_period;
                        editor_data.file.cursor = (cy+1, x.min(next_line_len));
                    },
                    x if x <= editor_data.indent => {
                        editor_data.file.cursor.1 = line_period + editor_data.indent + 1;
                    },
                    _ => {
                        let x = (cx + line_period).min(line_len);
                        editor_data.file.cursor.1 = x;
                    }
                },
                _ => unreachable!()
            };
            display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
        }
        Event::KeyPressedArrow((dy, 0), Modifiers::SHIFT) => {
            editor_data.file.camera = editor_data.file.camera
                .saturating_add_signed(dy)
                .clamp(1, editor_data.menu.lines.len());
            display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
        },
        Event::WindowResize(cols, rows) => {
            editor_data.dimensions = (cols, rows);
            editor_data.indent = (editor_data.dimensions.0 / 10).clamp(1, 4);
            display_file(&editor_data.file, &editor_data.dimensions, editor_data.indent);
        },
        _ => ()
    };
}
