use termwiz::{
    input::{
        InputEvent,
        KeyEvent, KeyCode, Modifiers,
        MouseEvent, MouseButtons
    },
    terminal::Terminal,
    Error
};

pub enum Event {
    KeyPressed(KeyCode, Modifiers),
    MouseMoved(u16, u16, MouseButtons, Modifiers),
    WindowResize(usize, usize),
    KeyboardInterrupt
}

pub fn poll_terminal_for_events(terminal: &mut impl Terminal) -> Result<Option<Event>, Error> {
    let Some(event) = terminal.poll_input(None)? else {
        return Ok(None);
    };
    let result = match event {
        InputEvent::Key(
            KeyEvent {
                key: KeyCode::Char('c'),
                modifiers: Modifiers::CTRL
            }
        ) => Some(Event::KeyboardInterrupt),
        InputEvent::Key(
            KeyEvent {
                key,
                modifiers
            }
        ) => Some(Event::KeyPressed(key, modifiers)),
        InputEvent::Mouse(
            MouseEvent {
                x, y, mouse_buttons, modifiers
            }
        ) => Some(Event::MouseMoved(x, y, mouse_buttons, modifiers)),
        InputEvent::Resized {
            cols, rows
        } => Some(Event::WindowResize(cols, rows)),
        _ => None
    };
    Ok(result)
}