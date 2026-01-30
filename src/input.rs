use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub enum InputEvent {
    Char(char),
    Escape,
    Enter,
    Up,
    Down,
    CtrlN,
    CtrlP,
    None,
}

pub fn read_key(timeout: Duration) -> Result<InputEvent, Box<dyn std::error::Error>> {
    if event::poll(timeout)? {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event::read()?
        {
            return Ok(match code {
                KeyCode::Char('n') if modifiers.contains(KeyModifiers::CONTROL) => InputEvent::CtrlN,
                KeyCode::Char('p') if modifiers.contains(KeyModifiers::CONTROL) => InputEvent::CtrlP,
                KeyCode::Char(c) => InputEvent::Char(c),
                KeyCode::Esc => InputEvent::Escape,
                KeyCode::Enter => InputEvent::Enter,
                KeyCode::Up => InputEvent::Up,
                KeyCode::Down => InputEvent::Down,
                _ => InputEvent::None,
            });
        }
    }
    Ok(InputEvent::None)
}
