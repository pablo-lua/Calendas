use std::{io::{self, Write, stdout}, time::Duration};
use crossterm::{
    queue,
    event::{read, KeyCode},
    ExecutableCommand, QueueableCommand,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap, enable_raw_mode, disable_raw_mode}, cursor::{self, MoveDown, MoveLeft, MoveRight, MoveUp}, style::{self, Stylize}, event::{Event, KeyEvent, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags}
};
use chrono::prelude::*;
use chrono::Duration as Duracao;

#[derive(Debug)]
pub struct Dia {
    dia: u8,
    dia_semana: Semana,
    mes: Mes,
    ano: Ano
} impl Dia {
    pub fn hoje() -> Self {
        let day = Local::now();
        Self { dia: day.day() as u8,
            dia_semana: Semana::from(day.weekday() as u8),
            mes: Mes::from(day.month() as u8),
            ano: Ano::Normal
        }
    }
}

#[derive(Clone, Debug)]
enum Mes {
    Janeiro,
    Fevereiro,
    Marco,
    Abril,
    Maio,
    Junho,
    Julho,
    Agosto,
    Setembro,
    Outubro,
    Novembro,
    Dezembro
} impl Into<u8> for Mes {
    fn into(self) -> u8 {
        return match self.clone() {
            Mes::Janeiro => 1,
            Mes::Fevereiro => 2,
            Mes::Marco => 3,
            Mes::Abril => 4,
            Mes::Maio => 5,
            Mes::Junho => 6,
            Mes::Julho => 7,
            Mes::Agosto => 8,
            Mes::Setembro => 9,
            Mes::Outubro=> 10,
            Mes::Novembro => 11,
            Mes::Dezembro => 12,
        }
    }
} impl From<u8> for Mes {
    fn from(value: u8) -> Self {
        return match value {
            1 => Self::Janeiro,
            2 => Self::Fevereiro,
            3 => Self::Marco,
            4 => Self::Abril,
            5 => Self::Maio,
            6 => Self::Junho,
            7 => Self::Julho,
            8 => Self::Agosto,
            9 => Self::Setembro,
            10 => Self::Outubro,
            11 => Self::Novembro,
            _ => Self::Dezembro,
        }
    }
} impl Mes {
    fn novo(num: u8) -> Self {
        return Self::from(num)
    }
}

#[derive(Debug)]
enum Ano {
    Normal,
    Bissexto
}

#[derive(Clone, Debug)]
enum Semana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo
} impl From<u8> for Semana {
    fn from(value: u8) -> Self {
        return match value {
            1 => Self::Segunda,
            2 => Self::Terca,
            3 => Self::Quarta,
            4 => Self::Quinta,
            5 => Self::Sexta,
            6 => Self::Sabado,
            _ => Self::Domingo,
        }
    }
} impl Into<String> for Semana {
    fn into(self) -> String {
        let dia_semana = match self.clone() {
            Self::Segunda => "Seg",
            Self::Terca => "Ter",
            Self::Quarta => "Qua",
            Self::Quinta => "Qui",
            Self::Sexta => "Sex",
            Self::Sabado => "Sab",
            Self::Domingo => "Dom",
        };
        return dia_semana.to_string();
    }
} impl Semana {

}
