use anyhow::Result;
use crossterm::{
    cursor, execute, queue, style,
    style::{Color, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use drawille::Canvas;
use std::io::{stdout, Stdout, Write};

use deca::Chip8;

#[derive(Copy, Clone)]
pub(crate) enum TerminalSize {
    Big = 0,
    Thin = 1,
    Small = 2,
    Smallest = 3,
    Braille = 4,
}

pub(crate) struct Terminal {
    stdout: Stdout,
    term_size: TerminalSize,
    colors: Vec<Color>,
}

impl Terminal {
    pub(crate) fn new(colors: Vec<Color>) -> Result<Self> {
        let mut terminal = Self {
            stdout: stdout(),
            term_size: TerminalSize::Big,
            colors,
        };
        execute!(terminal.stdout, EnterAlternateScreen, Clear(ClearType::All))?;
        enable_raw_mode()?;
        execute!(terminal.stdout, DisableLineWrap, cursor::Hide)?;
        Ok(terminal)
    }

    #[inline]
    pub(crate) fn resize(
        &mut self,
        (width, height): (u16, u16),
        (c8width, c8height): (u8, u8),
    ) -> Result<()> {
        self.term_size = if width >= u16::from(c8width) * 2 && height >= c8height.into() {
            TerminalSize::Big
        } else if width >= c8width.into() && height >= c8height.into() {
            TerminalSize::Thin
        } else if width >= c8width.into() && height >= u16::from(c8height) / 2 {
            TerminalSize::Small
        } else if width >= u16::from(c8width) / 2 && height >= u16::from(c8height) / 2 {
            TerminalSize::Smallest
        } else {
            TerminalSize::Braille
        };
        execute!(self.stdout, Clear(ClearType::All), cursor::Hide)?;
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn draw_display(&self, chip8: &Chip8) -> Result<()> {
        const BIG_CHARSET: &[&str] = &["  ", "██"];
        const THIN_CHARSET: &[&str] = &[" ", "█"];
        const SMALL_CHARSET: &[&str] = &[" ", "▄", "▀", "█"];
        const SMALLEST_CHARSET: &[&str] = &[
            " ", "▗", "▖", "▄", "▝", "▐", "▞", "▟", "▘", "▚", "▌", "▙", "▀", "▜", "▛", "█",
        ];

        const CHARSETS: [&[&str]; 4] = [BIG_CHARSET, THIN_CHARSET, SMALL_CHARSET, SMALLEST_CHARSET];

        let mut stdout = &self.stdout;
        let term_size = self.term_size;

        queue!(stdout, cursor::MoveTo(0, 0))?;
        queue!(stdout, style::SetBackgroundColor(self.colors[0]))?;
        queue!(stdout, style::SetForegroundColor(self.colors[1]))?;

        match term_size {
            TerminalSize::Big | TerminalSize::Thin => {
                for y in 0..chip8.display.height {
                    for x in 0..chip8.display.width {
                        let pixel = chip8.display.display[y as usize][x as usize] as usize;
                        queue!(
                            stdout,
                            style::Print(
                                CHARSETS[term_size as usize][if pixel > 0 { 1 } else { 0 }]
                                    .with(self.colors[pixel])
                                    .to_string()
                            )
                        )?;
                    }
                    //if y < chip8.display.height - 1 {
                    queue!(stdout, cursor::MoveToNextLine(0))?;
                    //}
                }
            }
            TerminalSize::Small => {
                for y in (0..chip8.display.height).step_by(2) {
                    for x in 0..chip8.display.width {
                        let pixels = (chip8.display.display[y as usize][x as usize] << 1)
                            | chip8.display.display[(y + 1) as usize][x as usize];
                        queue!(
                            stdout,
                            style::Print(SMALL_CHARSET[pixels as usize].to_string())
                        )?;
                    }
                    //if y < (chip8.display.height / 2) - 1 {
                    queue!(stdout, cursor::MoveToNextLine(0))?;
                    //}
                }
            }
            TerminalSize::Smallest => {
                for y in (0..chip8.display.height).step_by(2) {
                    for x in (0..chip8.display.width).step_by(2) {
                        let pixels = (chip8.display.display[y as usize][x as usize] << 3)
                            | (chip8.display.display[y as usize][(x + 1) as usize] << 2)
                            | (chip8.display.display[(y + 1) as usize][x as usize] << 1)
                            | chip8.display.display[(y + 1) as usize][(x + 1) as usize];
                        queue!(
                            stdout,
                            style::Print(SMALLEST_CHARSET[pixels as usize].to_string())
                        )?;
                    }
                    //if y < (chip8.display.height / 2) - 1 {
                    queue!(stdout, cursor::MoveToNextLine(0))?;
                    //}
                }
            }
            TerminalSize::Braille => {
                let mut canvas = Canvas::new(
                    (chip8.display.width - 1).into(),
                    (chip8.display.height - 1).into(),
                );
                for y in 0..chip8.display.height {
                    for x in 0..chip8.display.width {
                        if chip8.display.display[y as usize][x as usize] > 0 {
                            canvas.set(x.into(), y.into());
                        }
                    }
                }
                for row in canvas.rows() {
                    queue!(stdout, style::Print(row))?;
                    queue!(stdout, cursor::MoveToNextLine(0))?;
                }
            }
        }
        queue!(stdout, cursor::MoveTo(0, 0))?;
        stdout.flush()?;
        Ok(())
    }

    pub(crate) fn draw_debug(
        &mut self,
        chip8: &Chip8,
        interrupt: bool,
        halted: bool,
        halt_message: &str,
    ) -> Result<()> {
        if interrupt || halted {
            queue!(
                self.stdout,
                cursor::MoveTo(0, (chip8.display.height + 1).into())
            )?;
            queue!(self.stdout, style::ResetColor)?;
            if halted {
                queue!(self.stdout, style::SetForegroundColor(style::Color::Red))?;
                queue!(self.stdout, style::Print(halt_message.to_string()))?;
                queue!(self.stdout, cursor::MoveToNextLine(0))?;
                queue!(self.stdout, style::ResetColor)?;
            } else if interrupt {
                queue!(self.stdout, style::Print(halt_message.to_string()))?;
                queue!(self.stdout, cursor::MoveToNextLine(0))?;
            };
            queue!(
                self.stdout,
                style::Print(format!(
                    "PC: {:#06X} ({:#04x}{:02x})",
                    chip8.pc,
                    chip8.memory[chip8.pc as usize],
                    chip8.memory[chip8.pc as usize + 1],
                ))
            )?;
            queue!(self.stdout, cursor::MoveToNextLine(0))?;
            queue!(self.stdout, style::Print(format!("I: {:#06X}", chip8.i)))?;
            queue!(self.stdout, cursor::MoveToNextLine(0))?;
            for v in 0..16 {
                queue!(
                    self.stdout,
                    style::Print(format!("V{:X}: {:#04X}  ", v, chip8.v[v] as usize))
                )?;
            }
            //execute!(self.stdout, cursor::MoveToNextLine(0))?;
            //for v in 0..16 {
            //    execute!(
            //        self.stdout,
            //        style::Print(format!("K{:X}: {} ", v, chip8.keyboard[v]))
            //    )?;
            //}
        } else {
            queue!(self.stdout, style::ResetColor)?;
            queue!(
                self.stdout,
                cursor::MoveTo(0, (chip8.display.height + 1).into()),
                Clear(ClearType::FromCursorDown),
                cursor::MoveTo(0, 0),
            )?;
        }
        self.stdout.flush()?;
        Ok(())
    }
}

impl Drop for Terminal {
    #![allow(clippy::let_underscore_drop)]
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.stdout,
            LeaveAlternateScreen,
            EnableLineWrap,
            cursor::Show
        );
    }
}
