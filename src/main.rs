use ansi_colours::ansi256_from_rgb;
use clap::Parser;

use ini::Ini;

use deca::Chip8;
use octopt::{Options, Platform};

use dirs::{config_dir, home_dir};

use anyhow::Result;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    style,
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::time::Duration;
use std::u8;
use std::{
    io::{stdout, Write},
    path::PathBuf,
};

mod terminal;
use terminal::Terminal;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// CHIP-8 game file (binary ROM file, .o8 Octo source file, or .gif Octocart)
    #[clap(value_parser, value_name = "ROM")]
    rom: PathBuf,

    /// Instructions to execute per 60Hz frame
    #[clap(short, long, value_name = "TICKRATE", value_parser = clap::value_parser!(u16).range(1..))]
    tickrate: Option<u16>,

    /// Configuration file, compatible with C-Octo. In order, we will read the supplied file, a user-wide file in your home directory, and finally a file the same name and in the same location as the current ROM, but with an '.octo.rc' file extension, for easy per-game configuration.
    #[clap(short, long, value_name = "CONFIG_FILE", value_parser)]
    config: Option<PathBuf>,

    /// Symbol file, compatible with C-Octo
    #[clap(short, long, value_name = "SYMBOL_FILE")]
    symbols: Option<PathBuf>,

    /// Force quirky behavior for platform compatibility.
    /// (For fine-tuned quirks configuration, you can toggle individual settings in a configuration file; see --config)
    /// Possible values: vip, schip, octo
    #[clap(short, long, value_name = "COMPATIBILITY_PROFILE", arg_enum, default_value_t = Platform::Octo)]
    quirks: Platform,

    /// Starts execution in interrupted mode, for easier debugging
    #[clap(short, long)]
    debug: bool,
}

//#[allow(clippy::too_many_lines)]
fn main() -> Result<()> {
    let args = Args::parse();

    let rom = std::fs::read(args.rom)?;

    let mut chip8 = Chip8::new(Options::new(args.quirks));

    let mut stdout = stdout();

    if let Some(max_size) = chip8.options.max_size {
        if rom.len() > max_size as usize {
            println!("Warning: ROM size ({}) exceeds maximum available memory on target platform {} ({}). Will not run on real hardware.", rom.len(), args.quirks, max_size);
            print!("Press any key to run it anyway. ");
            stdout.flush()?;
            enable_raw_mode()?;
            let _key = read()?;
            disable_raw_mode()?;
            println!();
        }
    };

    chip8.read_rom(&rom);

    // TODO this can be better. Maybe use figment?
    let tickrate = match args.tickrate {
        Some(s) => s,
        None => chip8.options.tickrate.unwrap_or(500),
    };

    let conf = Ini::load_from_file("/home/tvl/.octo.rc")?; // FIXME
    let section = conf
        .section(None::<String>)
        .expect("Failed to load top-level section of .rc file; this should be impossible");

    let colors = vec![
        color_from_ini(section, "color.plane0").unwrap_or(style::Color::Black),
        color_from_ini(section, "color.plane1").unwrap_or(style::Color::White),
        color_from_ini(section, "color.plane2").unwrap_or(style::Color::Red),
        color_from_ini(section, "color.plane3").unwrap_or(style::Color::Green),
    ];

    let mut terminal = Terminal::new(colors)?;
    terminal.resize(size()?, (chip8.display.width, chip8.display.height))?;

    let mut interrupt = args.debug;
    let mut halted = false;
    let mut halt_message = "".to_string();

    'outer: loop {
        if !interrupt && !halted {
            if let Err(error) = chip8.run(tickrate) {
                halted = true;
                halt_message = error;
            }
        }

        for key in &mut chip8.keyboard {
            *key = false;
        }
        while poll(Duration::from_millis(100))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(keyevent) => match keyevent.code {
                    KeyCode::Esc => break 'outer,
                    KeyCode::Char('1') => chip8.keyboard[0x1] = true,
                    KeyCode::Char('2') => chip8.keyboard[0x2] = true,
                    KeyCode::Char('3') => chip8.keyboard[0x3] = true,
                    KeyCode::Char('4') => chip8.keyboard[0xC] = true,
                    KeyCode::Char('q' | ' ') => chip8.keyboard[0x4] = true,
                    KeyCode::Char('w') | KeyCode::Up => chip8.keyboard[0x5] = true,
                    KeyCode::Char('e') => chip8.keyboard[0x6] = true,
                    KeyCode::Char('r') => chip8.keyboard[0xD] = true,
                    KeyCode::Char('a') | KeyCode::Left => chip8.keyboard[0x7] = true,
                    KeyCode::Char('s') | KeyCode::Down => chip8.keyboard[0x8] = true,
                    KeyCode::Char('d') | KeyCode::Right => chip8.keyboard[0x9] = true,
                    KeyCode::Char('f') => chip8.keyboard[0xE] = true,
                    KeyCode::Char('z') => chip8.keyboard[0xA] = true,
                    KeyCode::Char('x') => chip8.keyboard[0x0] = true,
                    KeyCode::Char('c') => {
                        if keyevent.modifiers.contains(KeyModifiers::CONTROL) {
                            break 'outer;
                        }
                        chip8.keyboard[0xB] = true;
                    }
                    KeyCode::Char('v') => chip8.keyboard[0xF] = true,
                    KeyCode::Char('i') => {
                        if interrupt {
                            terminal.erase_debug(&chip8)?;
                        } else {
                            halt_message = "user interrupt".to_string();
                        }
                        interrupt = !interrupt;
                    }
                    KeyCode::Char('o') => {
                        if interrupt && !halted {
                            halt_message = match chip8.run(1) {
                                Err(error) => {
                                    halted = true;
                                    error
                                }
                                Ok(_) => "single stepping".to_string(),
                            }
                        }
                    }
                    KeyCode::Char('m') => todo!(), // TODO Display memory monitors
                    _ => (),
                },
                Event::Resize(width, height) => {
                    chip8.display.dirty = terminal
                        .resize((width, height), (chip8.display.width, chip8.display.height))?;
                }
                Event::Mouse(_) => todo!(),
                _ => (),
            }
        }

        if chip8.display.dirty {
            chip8.display.dirty = false;
            terminal.draw_display(&chip8)?;
        }

        if interrupt || halted {
            terminal.draw_debug(&chip8, interrupt, halted, &halt_message)?;
        }
        // TODO play sound if chip8.sound is greater than 0
    }

    Ok(())
}

fn color_from_ini(section: &ini::Properties, attribute: &str) -> Option<style::Color> {
    let mut v = vec![];
    let mut cur = section.get(attribute)?;

    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(std::cmp::min(2, cur.len()));
        v.push(chunk);
        cur = rest;
    }

    let rgb = (
        u8::from_str_radix(v[0], 16).ok()?,
        u8::from_str_radix(v[1], 16).ok()?,
        u8::from_str_radix(v[2], 16).ok()?,
    );

    if style::available_color_count() > 256 {
        Some(style::Color::Rgb {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        })
    } else {
        Some(style::Color::AnsiValue(ansi256_from_rgb(rgb)))
    }
}
