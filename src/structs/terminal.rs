use std::{io::{self, Write, stdout, Stdout}, time::Duration, thread};
use crossterm::{
    queue,
    event::{read, KeyCode},
    ExecutableCommand, QueueableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap, enable_raw_mode, disable_raw_mode, window_size}, cursor::{self, MoveDown, MoveLeft, MoveRight, MoveUp, MoveTo}, style::{self, Stylize}, event::{Event, KeyEvent, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags}
};

pub struct Terminal {
    window_size: (u32, u32),
    stdout: Stdout
} impl Terminal {
    pub fn start() -> io::Result<Self> {
        let mut stdout = stdout();
        stdout.queue(EnterAlternateScreen)?;
        Ok(Self { window_size: (30, 30), stdout})
    }
    pub fn matar(mut self) -> io::Result<()> {
        self.stdout.queue(LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn foo(&mut self, tam: u32, pos: (u32, u32))  -> io::Result<()>{
        quadrado(self, tam, pos)?;
        thread::sleep(Duration::from_secs(2));
        stdout().queue(LeaveAlternateScreen)?;
        Ok(())
    }
    fn flush(&mut self) -> io::Result<()>{
        self.stdout.flush()
    }

    fn init_eventos(&mut self) -> io::Result<bool> {
        enable_raw_mode()?;

        let supports_keyboard_enhancement = matches!(
            crossterm::terminal::supports_keyboard_enhancement(),
            Ok(true)
        );

        if supports_keyboard_enhancement {
            queue!(
                self.stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                )
            )?;
        }
        Ok(supports_keyboard_enhancement)
    }

    fn eventos(&mut self) -> io::Result<()>{
        let a = self.init_eventos()?;
        loop {
            match read()? {
                Event::Key(k) => {
                    match k.code {
                        KeyCode::Esc => {break}
                        _ => {}
                    }
                }
                _ => {faz_nada()}
            }
        };
        disable_raw_mode()?;
        Ok(())
    }

    pub fn init(&mut self) -> io::Result<()>{
        let tam = window_size()?;
        let (x_max, y_max) = (tam.columns, tam.rows);
        for x in 0..=x_max {
            for y in 0..=y_max {
                if (x % (x_max / 7) == 0 || y % (y_max / 5) == 0) && (y <= y_max - 2) {
                    self.stdout.queue(MoveTo(x, y))?;
                    print!(".");
                    self.flush()?;
                }
            }
        }
        self.eventos();
        Ok(())
    }
    pub fn init_q(&mut self) {
        quadrado(self, 10, (0, 0));
    }
}

fn quadrado(ter: &mut Terminal, tam: u32, pos: (u32, u32)) -> io::Result<()>{
    for x in [pos.0, pos.0 + tam]  {
        for y in pos.1..pos.1+tam {
            print!("󰤃");
            ter.stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
            ter.flush()?;
        }
    }
    for y in [pos.1, pos.1 + tam] {
        for x in pos.0..pos.0+tam {
            ter.stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
            print!("");
            ter.flush()?;
        }
    }
    Ok(())
}

fn ponto_no_retangulo(inicio: (u16, u16), tam: (u16, u16), ponto: (u16, u16)) -> bool {
    return (inicio.0 <= ponto.0 && ponto.0 <= inicio.0 + tam.0) && (inicio.1 <= ponto.1 && ponto.1 <= inicio.1 + tam.1);
}

fn faz_nada() {

}
