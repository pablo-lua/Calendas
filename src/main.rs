use std::{io::{self, Write, stdout}, time::Duration, thread};
use crossterm::{
    queue,
    event::{read, KeyCode},
    ExecutableCommand, QueueableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap, enable_raw_mode, disable_raw_mode}, cursor::{self, MoveDown, MoveLeft, MoveRight, MoveUp}, style::{self, Stylize}, event::{Event, KeyEvent, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags}
};

mod structs;

fn main() -> io::Result<()> {
    let mut a = structs::terminal::Terminal::start()?;
    a.init()?;
    a.matar()?;
    Ok(())
}

fn print_events() -> io::Result<()>{
    println!("Começando agora aos eventos:");
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Esc => {break;}
                    KeyCode::Down => {
                        stdout().queue(MoveDown(1))?.flush()?;
                    }
                    KeyCode::Up => {
                        stdout().queue(MoveUp(1))?.flush()?;
                    }
                    KeyCode::Left => {
                        stdout().queue(MoveLeft(1))?.flush()?;
                    }
                    KeyCode::Right => {
                        stdout().queue(MoveRight(1))?.flush()?;
                    }
                    KeyCode::Char(c) => {print!("{}", c); stdout().flush()?;}
                    _ => {println!("{:?}\r", event.code)}
                }
            },
            _ => {}

        }
    }
    Ok(())
}
