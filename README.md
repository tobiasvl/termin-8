Termin-8
========
[![crates.io](https://img.shields.io/crates/v/termin-8.svg)](https://crates.io/crates/termin-8)

Octo-compliant CHIP-8 emulator frontend that runs in your terminal.

It uses [`deca`](https://crates.io/crates/deca) as the emulator backend, which supports CHIP-8, SUPER-CHIP (SCHIP) and XO-CHIP programs.

Note that the terminal size requirements vary depending on what kind of program you attempt to run.

### CHIP-8

For running regular, 64x32 CHIP-8 programs, your options are the following:

* 128 x 32 terminal size, for large and square pixels
* 64 x 32 terminal size, for rectangular pixels
* 64 x 16 terminal size and a font that supports Unicode Block Elements characters, for small and square pixels
* 32 x 16 terminal size and a font that supports the expanded Unicode Block Elements character set, for small and rectangular pixels

### SUPER-CHIP

For running 128 x 64 SUPER-CHIP programs:

* 256 x 64 terminal size, for large and square pixels
* 128 x 64 terminal size, for rectangular pixels
* 128 x 32 terminal size and a font that supports Unicode Block Elements characters, for small and square pixels
* 64 x 32 terminal size and a font that supports the expanded Unicode Block Elements character set, for small and rectangular pixels

### XO-CHIP

For running 128 x 64 XO-CHIP programs with color:

* 256 x 64 terminal size, for large and square pixels with color support
* 128 x 64 terminal size, for rectangular pixels with color support
* 128 x 32 terminal size and a font that supports Unicode Block Elements characters, for small and square pixels with color support

If you don't need color support:

* 64 x 32 terminal size and a font that supports the expanded Unicode Block Elements character set, for small and rectangular pixels

Note that most XO-CHIP programs use color.

## Installation

```sh
cargo install termin-8
```

## Usage

```sh
termin-8 ROM
``` 
