extern crate ansi_colours;
use ansi_colours::*;

extern crate ini;
use ini::Ini;

extern crate clap;
use clap::{crate_version, App, Arg};

//mod chip8;
//use chip8::Chip8;
//use chip8::Quirks;
extern crate deca;
use deca::Chip8;
use deca::Quirks;

//extern crate drawille;
//use drawille::Canvas;

extern crate dirs;
use dirs::{config_dir, home_dir};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, style, terminal,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
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
                .help("Configuration file, compatible with C-Octo")
                .default_value("~/.octo.rc")
        )
        .arg(Arg::with_name("quirks")
                .short("q")
                .long("quirks")
                .takes_value(true)
                .value_name("COMPATIBILITY_PROFILE")
                .help("Force quirky behavior for platform compatibility.\n(For fine-tuned quirks configuration, you can toggle individual settings in a configuration file; see --config)\nPossible values: chip8, schip, octo")
                .default_value("octo")
        )
        .arg(
            Arg::with_name("ROM")
                .help("CHIP-8 ROM file")
                .required(true) // for the time being
                //.index(1),
        )
        .get_matches();

    let rom = std::fs::read(matches.value_of("ROM").unwrap()).expect("Couldn't load ROM");

    let quirks = match matches.value_of("quirks").unwrap() {
        "vip" => Quirks {
            shift: false,
            loadstore: false,
            jump0: false,
            logic: true,
            clip: true,
            vblank: true,
            resclear: false,
            delaywrap: false,
            multicollision: false,
            loresbigsprite: false,
            lorestallsprite: false,
            max_rom: 3216,
        },
        "schip" => Quirks {
            shift: true,
            loadstore: true,
            jump0: true,
            logic: false,
            clip: true,
            vblank: false,
            resclear: false,
            delaywrap: false,
            multicollision: false,
            loresbigsprite: false,
            lorestallsprite: false,
            max_rom: 3583,
        },
        _ => Quirks {
            shift: false,
            loadstore: false,
            jump0: false,
            logic: false,
            clip: false,
            vblank: false,
            resclear: false,
            delaywrap: false,
            multicollision: false,
            loresbigsprite: false,
            lorestallsprite: false,
            max_rom: 65024,
        },
    };

    let mut chip8 = Chip8::new(quirks);

    if rom.len() > chip8.quirks.max_rom as usize {
        println!("Warning: ROM size ({}) exceeds maximum available memory on target platform ({}) (will try to run anyway)", rom.len(), chip8.quirks.max_rom)
    }

    chip8.read_rom(&rom);

    let tickrate = match matches.value_of("num") {
        None => 400,
        Some(s) => s.parse::<u16>().unwrap_or(400),
    };

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen, Clear(ClearType::All)).unwrap();
    enable_raw_mode().unwrap();
    execute!(stdout, DisableLineWrap, cursor::Hide).unwrap();

    let conf = Ini::load_from_file("/home/tvl/.octo.rc").unwrap();
    let section = conf.section(None::<String>).unwrap();

    let bg_color = color_from_ini(section, "color.plane0");
    let fg_color = color_from_ini(section, "color.plane1");

    execute!(stdout, style::SetBackgroundColor(bg_color)).unwrap();
    execute!(stdout, style::SetForegroundColor(fg_color)).unwrap();

    let big_charset = vec!["  ", "██"];
    let thin_charset = vec![" ", "█"];
    let small_charset = vec![" ", "▄", "▀", "█"];
    let smallest_charset = vec![
        " ", "▗", "▖", "▄", "▝", "▐", "▞", "▟", "▘", "▚", "▌", "▙", "▀", "▜", "▛", "█",
    ];
    let (width, height) = size().unwrap();
    let mut charset = if width >= chip8.display.width * 2 && height >= chip8.display.height {
        &big_charset
    } else {
        //if width >= chip8.display.width && height >= chip8.display.height {
        &thin_charset
        //    } else if width >= chip8.display.width / 2 && height >= chip8.display.height {
        //        &small_charset
        //    } else {
        //        &smallest_charset
    };

    let mut interrupt = true;
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
        loop {
            if poll(Duration::from_millis(1)).unwrap() {
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
                        charset =
                            if width >= chip8.display.width * 2 && height >= chip8.display.height {
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
            } else {
                break;
            }
        }

        if chip8.display.dirty {
            let (width, height) = size().unwrap();

            execute!(
                stdout,
                terminal::SetTitle(format!(
                    "{}x{} actual, {}x{} c8, {} colors",
                    width.to_string(),
                    height.to_string(),
                    chip8.display.width.to_string(),
                    chip8.display.height.to_string(),
                    style::available_color_count()
                ))
            )
            .unwrap();

            // draw to terminal
            queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
            execute!(stdout, style::SetBackgroundColor(bg_color)).unwrap();
            execute!(stdout, style::SetForegroundColor(fg_color)).unwrap();

            if width >= chip8.display.width && height >= chip8.display.height {
                for y in 0..chip8.display.height {
                    for x in 0..chip8.display.width {
                        queue!(
                            stdout,
                            style::Print(
                                charset[chip8.display.display[y as usize][x as usize] as usize]
                                    .to_string()
                            )
                        )
                        .unwrap();
                    }
                    queue!(stdout, cursor::MoveToNextLine(0)).unwrap();
                }
            } else if width >= chip8.display.width && height >= chip8.display.height / 2 {
                for y in (0..chip8.display.height).step_by(2) {
                    for x in 0..chip8.display.width {
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
        if interrupt || halted {
            execute!(stdout, cursor::MoveTo(0, chip8.display.height + 1)).unwrap();
            execute!(stdout, style::ResetColor).unwrap();
            if halted {
                execute!(stdout, style::SetForegroundColor(style::Color::Red)).unwrap();
                execute!(stdout, style::Print(halt_message.to_string())).unwrap();
                execute!(stdout, cursor::MoveToNextLine(0)).unwrap();
                execute!(stdout, style::ResetColor).unwrap();
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
                cursor::MoveTo(0, chip8.display.height + 1),
                Clear(ClearType::FromCursorDown)
            )
            .unwrap();
        }
        // play sound if chip8.sound is greater than 0
    }
}

fn color_from_ini(section: &ini::Properties, attribute: &str) -> style::Color {
    let mut v = vec![];
    let mut cur = section.get(attribute).unwrap();
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
        style::Color::Rgb {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    } else {
        style::Color::AnsiValue(ansi256_from_rgb(rgb))
    }
}

fn exit() {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, EnableLineWrap, cursor::Show).unwrap();
    std::process::exit(0);
}