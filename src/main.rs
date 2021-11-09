use ansi_colours::*;

use ini::Ini;

use clap::{crate_version, App, Arg};

use deca::*;
use octopt::*;

//extern crate drawille;
//use drawille::Canvas;

extern crate dirs;
use dirs::{config_dir, home_dir};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute, queue, style,
    style::Stylize,
    terminal,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io::{stdout, Write};
use std::time::Duration;
use std::u8;

fn main() {
    let matches = App::new("Termin-8")
        .version(crate_version!())
        .author("Tobias V. Langhoff <tobias@langhoff.no>")
        .about("Octo emulator")
        .arg(Arg::with_name("tickrate")
                .short("t")
                .long("tickrate")
                .takes_value(true)
                .value_name("TICKRATE")
                .help("Instructions to execute per 60Hz frame")
                .default_value("40")
        )
        .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("CONFIG_FILE")
                .help("Configuration file, compatible with C-Octo\nIf not supplied, we will attempt to find a file with the same name and in the same location as the current ROM, but with an '.octo.rc' file extension, for easy per-game configuration.\nIf that doesn't exist, the default is ~/.octo.rc")
                .default_value("~/.octo.rc")
        )
        .arg(Arg::with_name("quirks")
                .short("q")
                .long("quirks")
                .takes_value(true)
                .value_name("COMPATIBILITY_PROFILE")
                .help("Force quirky behavior for platform compatibility.\n(For fine-tuned quirks configuration, you can toggle individual settings in a configuration file; see --config)\nPossible values: vip, schip, octo")
                .default_value("octo")
        )
        .arg(Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Starts execution in interrupted mode, for easier debugging")
        )
        .arg(
            Arg::with_name("ROM")
                .help("CHIP-8 game file (binary ROM file, .o8 Octo source file, or .gif Octocart)")
                .required(true) // for the time being
                //.index(1),
        )
        .get_matches();

    let rom = std::fs::read(matches.value_of("ROM").unwrap()).expect("Couldn't load ROM");

    let platform = match matches.value_of("quirks").unwrap() {
        "vip" => Platform::Vip,
        "schip" => Platform::Schip,
        _ => Platform::Octo,
    };

    let mut chip8 = Chip8::new(platform);

    if let Some(max_size) = chip8.options.max_size {
        if rom.len() > max_size as usize {
            println!("Warning: ROM size ({}) exceeds maximum available memory on target platform ({}). Will not run on real hardware.", rom.len(), max_size);
            println!("Press any key to run it anyway.");
            let _ = read();
        }
    };

    chip8.read_rom(&rom);

    let tickrate = match matches.value_of("tickrate") {
        Some(s) => s.parse::<u16>().unwrap_or(400),
        _ => panic!("Tickrate must be a number"),
    };

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
    enable_raw_mode().unwrap();
    execute!(stdout, DisableLineWrap, cursor::Hide).unwrap();

    let conf = Ini::load_from_file("/home/tvl/.octo.rc").unwrap();
    let section = conf.section(None::<String>).unwrap();

    let colors = vec![
        color_from_ini(section, "color.plane0").unwrap_or(style::Color::Black),
        color_from_ini(section, "color.plane1").unwrap_or(style::Color::White),
        color_from_ini(section, "color.plane2").unwrap_or(style::Color::Red),
        color_from_ini(section, "color.plane3").unwrap_or(style::Color::Green),
    ];

    let big_charset = vec!["  ", "██"];
    let thin_charset = vec![" ", "█"];
    let small_charset = vec![" ", "▄", "▀", "█"];
    let smallest_charset = vec![
        " ", "▗", "▖", "▄", "▝", "▐", "▞", "▟", "▘", "▚", "▌", "▙", "▀", "▜", "▛", "█",
    ];

    let (width, height) = size().unwrap();
    let mut charset =
        if width >= (chip8.display.width * 2).into() && height >= chip8.display.height.into() {
            &big_charset
        } else {
            //if width >= chip8.display.width && height >= chip8.display.height {
            &thin_charset
            //    } else if width >= chip8.display.width / 2 && height >= chip8.display.height {
            //        &small_charset
            //    } else {
            //        &smallest_charset
        };

    let mut interrupt = matches.is_present("debug");
    let mut halted = false;

    loop {
        let mut halt_message = if !interrupt && !halted {
            match chip8.run(tickrate) {
                Err(error) => {
                    halted = true;
                    error
                }
                Ok(_) => "".to_string(),
            }
        } else {
            "".to_string()
        };

        // check for debug keypress
        // check for regular keypress?
        for key in chip8.keyboard.iter_mut() {
            *key = false;
        }
        while poll(Duration::from_millis(1)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::Key(keyevent) => match keyevent.code {
                    KeyCode::Esc => exit(),
                    KeyCode::Char('1') => chip8.keyboard[0x1] = true,
                    KeyCode::Char('2') => chip8.keyboard[0x2] = true,
                    KeyCode::Char('3') => chip8.keyboard[0x3] = true,
                    KeyCode::Char('4') => chip8.keyboard[0xC] = true,
                    KeyCode::Char('q') | KeyCode::Char(' ') => chip8.keyboard[0x4] = true,
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
                            exit()
                        } else {
                            chip8.keyboard[0xB] = true
                        }
                    }
                    KeyCode::Char('v') => chip8.keyboard[0xF] = true,
                    KeyCode::Char('i') => interrupt = !interrupt,
                    KeyCode::Char('o') => {
                        if interrupt && !halted {
                            halt_message = match chip8.run(1) {
                                Err(error) => {
                                    halted = true;
                                    error
                                }
                                Ok(_) => "".to_string(),
                            }
                        }
                    }
                    KeyCode::Char('m') => (),
                    _ => (),
                },
                Event::Resize(width, height) => {
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                    execute!(stdout, cursor::Hide).unwrap();
                    chip8.display.dirty = true;
                    charset = if width >= (chip8.display.width * 2).into()
                        && height >= chip8.display.height.into()
                    {
                        &big_charset
                    } else {
                        // if width >= chip8.display.width && height >= chip8.display.height {
                        &thin_charset
                        //} else if width >= chip8.display.width / 2 && height >= chip8.display.height {
                        //    &small_charset
                        //} else {
                        //    &smallest_charset
                    };
                }
                _ => (),
            }
        }

        if chip8.display.dirty {
            chip8.display.dirty = false;
            let (width, height) = size().unwrap();

            execute!(
                stdout,
                terminal::SetTitle(format!(
                    "{}x{} actual, {}x{} c8, {} colors, {} ticks",
                    width.to_string(),
                    height.to_string(),
                    chip8.display.width.to_string(),
                    chip8.display.height.to_string(),
                    style::available_color_count(),
                    tickrate
                ))
            )
            .unwrap();

            // draw to terminal
            queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
            execute!(stdout, style::SetBackgroundColor(colors[0])).unwrap();
            execute!(stdout, style::SetForegroundColor(colors[1])).unwrap();

            if width >= chip8.display.width.into() && height >= chip8.display.height.into() {
                for y in 0..chip8.display.height {
                    for x in 0..chip8.display.width.into() {
                        let pixel = chip8.display.display[y as usize][x as usize] as usize;
                        queue!(
                            stdout,
                            style::Print(
                                charset[if pixel > 0 { 1 } else { 0 }]
                                    .with(colors[pixel])
                                    .to_string()
                            )
                        )
                        .unwrap();
                    }
                    queue!(stdout, cursor::MoveToNextLine(0)).unwrap();
                }
            } else if width >= chip8.display.width.into()
                && height >= (chip8.display.height / 2).into()
            {
                for y in (0..chip8.display.height).step_by(2) {
                    for x in 0..chip8.display.width.into() {
                        let pixels = (chip8.display.display[y as usize][x as usize] << 1)
                            | chip8.display.display[(y + 1) as usize][x as usize];
                        queue!(
                            stdout,
                            style::Print(small_charset[pixels as usize].to_string())
                        )
                        .unwrap();
                    }
                    queue!(stdout, cursor::MoveToNextLine(0)).unwrap();
                }
            } else {
                //let mut canvas =
                //    Canvas::new(chip8.display.width as u32, chip8.display.height as u32);
                //canvas.set(5, 4);
                //for y in (0..chip8.display.height) {
                //    for x in (0..chip8.display.width) {
                //        if chip8.display.display[y as usize][x as usize] == 1 {
                //            canvas.set(x as u32, y as u32);
                //        }
                //    }
                //}
                //execute!(stdout, style::Print(canvas.frame()));

                for y in (0..chip8.display.height).step_by(2) {
                    for x in (0..chip8.display.width).step_by(2) {
                        let pixels = (chip8.display.display[y as usize][x as usize] << 3)
                            | (chip8.display.display[y as usize][(x + 1) as usize] << 2)
                            | (chip8.display.display[(y + 1) as usize][x as usize] << 1)
                            | chip8.display.display[(y + 1) as usize][(x + 1) as usize];
                        queue!(
                            stdout,
                            //style::SetBackgroundColor(bg_color),
                            style::Print(smallest_charset[pixels as usize].to_string())
                        )
                        .unwrap();
                    }
                    queue!(stdout, cursor::MoveToNextLine(0)).unwrap();
                }
            };
            stdout.flush().unwrap();
        }
        if interrupt || halted || true {
            execute!(stdout, cursor::MoveTo(0, (chip8.display.height + 1).into())).unwrap();
            execute!(stdout, style::ResetColor).unwrap();
            if halted {
                execute!(stdout, style::SetForegroundColor(style::Color::Red)).unwrap();
                execute!(stdout, style::Print(halt_message.to_string())).unwrap();
                execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
                execute!(stdout, style::ResetColor).unwrap();
            } else if interrupt {
                execute!(stdout, style::Print("user interrupt".to_string())).unwrap();
                execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
            };
            execute!(
                stdout,
                style::Print(format!(
                    "PC: {:#06X} ({:#04x}{:02x})",
                    chip8.pc,
                    chip8.memory[chip8.pc as usize],
                    chip8.memory[chip8.pc as usize + 1],
                ))
            )
            .unwrap();
            execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
            execute!(stdout, style::Print(format!("I: {:#06X}", chip8.i))).unwrap();
            execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
            for v in 0..16 {
                execute!(
                    stdout,
                    style::Print(format!("V{:X}: {:#04X}  ", v, chip8.v[v] as usize))
                )
                .unwrap();
            }
            execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
            for v in 0..16 {
                execute!(
                    stdout,
                    style::Print(format!("K{:X}: {} ", v, chip8.keyboard[v]))
                )
                .unwrap();
            }
        } else {
            execute!(stdout, style::ResetColor).unwrap();
            execute!(
                stdout,
                cursor::MoveTo(0, (chip8.display.height + 1).into()),
                Clear(ClearType::FromCursorDown)
            )
            .unwrap();
        }
        // play sound if chip8.sound is greater than 0
    }
}

fn color_from_ini(section: &ini::Properties, attribute: &str) -> Option<style::Color> {
    let mut v = vec![];
    let cur = section.get(attribute);
    cur?;
    let mut cur = cur.unwrap();
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(std::cmp::min(2, cur.len()));
        v.push(chunk);
        cur = rest;
    }

    let rgb = (
        u8::from_str_radix(v[0], 16).unwrap(),
        u8::from_str_radix(v[1], 16).unwrap(),
        u8::from_str_radix(v[2], 16).unwrap(),
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

fn exit() {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, EnableLineWrap, cursor::Show).unwrap();
    std::process::exit(0);
}
