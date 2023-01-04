use std::collections::HashMap;
use termwiz::{
    caps::Capabilities,
    terminal::{new_terminal, Terminal},
    input::{KeyCode, Modifiers},
    Error
};

mod events;
use events::{poll_terminal_for_events, Event};

mod graphics;
use graphics::{
    get_menu, MenuGraphics, MenuLine,
    get_file, FileGraphics,
    display_menu, display_file
};

mod parsing;

mod library_data;
use library_data::{
    Reference, LibraryData,
    read_lib_data
};

struct EditorData {
    state: EditorState,
    menu: MenuGraphics,
    file: FileGraphics,
    lib_data: LibraryData,
    references: HashMap<String, Reference>
}

enum EditorState {
    InMenu,
    EditingFile,
    ShouldExit
}

fn handle_event_in_menu(event: Event, editor_data: &mut EditorData) {
    match event {
        Event::KeyboardInterrupt => editor_data.state = EditorState::ShouldExit,
        Event::KeyPressed(KeyCode::UpArrow, Modifiers::NONE) => {
            if editor_data.menu.cursor <= 2 { return; };
            editor_data.menu.cursor -= 1;
            display_menu(&editor_data.menu);
        },
        Event::KeyPressed(KeyCode::DownArrow, Modifiers::NONE) => {
            if editor_data.menu.cursor >= editor_data.menu.lines.len() { return; };
            editor_data.menu.cursor += 1;
            display_menu(&editor_data.menu);
        },
        Event::KeyPressed(KeyCode::Enter, Modifiers::NONE) => {
            let index = editor_data.menu.cursor - 1;
            if let Some(MenuLine::File(_, path, _, _)) = editor_data.menu.lines.get(index) {
                editor_data.file = get_file(
                    path.to_owned(), &editor_data.lib_data, &editor_data.references
                ).unwrap();
                editor_data.state = EditorState::EditingFile;
                display_file(&editor_data.file);
            };
        },
        _ => ()
    };
}

fn handle_event_in_file_edition(event: Event, editor_data: &mut EditorData) {
    match event {
        Event::KeyPressed(KeyCode::Escape, Modifiers::NONE) => {
            editor_data.state = EditorState::InMenu;
            display_menu(&editor_data.menu);
        },
        _ => ()
    };
}

fn main() -> Result<(), Error> {
    let caps = Capabilities::new_from_env()?;
    let mut terminal = new_terminal(caps)?;
    terminal.set_raw_mode()?;

    let (lib_data, references) = read_lib_data().unwrap();
    let mut editor_data = EditorData {
        state: EditorState::InMenu,
        menu: get_menu().unwrap(),
        file: FileGraphics::default(),
        lib_data,
        references
    };
    display_menu(&editor_data.menu);

    loop {
        let Some(event) = poll_terminal_for_events(&mut terminal)? else {
            continue;
        };
        match editor_data.state {
            EditorState::InMenu => handle_event_in_menu(event, &mut editor_data),
            EditorState::EditingFile => handle_event_in_file_edition(event, &mut editor_data),
            EditorState::ShouldExit => break
        };
        match editor_data.state {
            EditorState::ShouldExit => break,
            _ => ()
        }
    };
    Ok(())
}
