Termin-8
========
[![crates.io](https://img.shields.io/crates/v/termin-8.svg)](https://crates.io/crates/termin-8)
[![dependency status](https://deps.rs/repo/github/tobiasvl/termin-8/status.svg)](https://deps.rs/crate/termin-8)

CHIP-8 emulator that runs in your terminal and is [Octo](https://github.com/JohnEarnest/Octo) compliant.

Powered by [`deca`](https://crates.io/crates/deca), it supports CHIP-8, SUPER-CHIP (SCHIP) and XO-CHIP programs.

It will also auto-resize its display to fit your terminal (at the cost of widespread font support and XO-CHIP color in the smallest sizes).

## Installation

First, [install Rust](https://www.rust-lang.org/tools/install). Then, in your terminal:

```sh
cargo install termin-8
```

## Usage

Termin-8 should work on Windows, Linux, and macOS. If it doesn't, please [file an issue](https://github.com/tobiasvl/termin-8/issues/new/choose).

Download CHIP-8 ROMs from the internet, like the [Chip8 Community Archive](https://github.com/JohnEarnest/chip8Archive/), or make your own in [Octo](https://github.com/JohnEarnest/Octo).

Then run Termin-8 in your terminal:

```sh
termin-8 ROM
```

There are some command line options:

```sh
termin-8 --help
```

The hexadecimal CHIP-8 keypad is customarily mapped to the following keyboard keys:

|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| q | w | e | r |
| a | s | d | f |
| z | x | c | v |

In addition, the arrow keys are bound to WASD.

You can press <kbd>Esc</kbd> to exit.

## Debugging capabilities

Termin-8 can be used for testing while developing CHIP-8 games, as an alternative to [Octo](https://JohnEarnest.github.io/Octo) (web) and [C-Octo](https://github.com/JohnEarnest/C-Octo) (SDL).

However, Termin-8 can't yet compile Octo code on its own. In an all-terminal workflow, use C-Octo's [`octo-cli`](https://github.com/JohnEarnest/c-octo#octo-cli) tool to compile your code. `octo-cli` can output a symbol file alongside the CHIP-8 binary if you use the `-s` command line option, and `-s` is likewise supported by Termin-8 for reading such a symbol file.

If a symbol file containing breakpoints is loaded, hitting those breakpoints will interrupt execution and display the contents of all registers.

Press the following keys while Termin-8 is running for further debugging:

* <kbd>i</kbd>: interrupt execution and display contents of registers (or continue execution after interrupt)
* <kbd>o</kbd>: single-step (while interrupted)
* <kbd>Esc</kbd>: exit

## Terminal requirements

Note that the terminal requirements vary depending on what kind of program you attempt to run.

Here's a table with the required terminal size and Unicode support needed to get features such as XO-CHIP color support, depending on the resolution of the CHIP-8 program you're running and the [Unicode Block Elements](https://en.wikipedia.org/wiki/Block_Elements) support of your font:

<table>
<tr>
<th>CHIP-8 resolution</td>
<th>Unicode version</td>
<th>Minimum terminal size</td>
<th>Pixel size</td>
<th>XO-CHIP colors</td>
</tr>

<tr>
<td rowspan="5">64x32 (lores)<br>CHIP-8, SCHIP, XO-CHIP</td>
<td rowspan="3">1.0.0</td>
<td>128x32</td>
<td>██</td>
<td>✔</td>
</tr>

<tr>
<td>64x32</td>
<td>█</td>
<td>✔</td>
</tr>

<tr>
<td>64x16</td>
<td>▀</td>
<td>✔</td>
</tr>

<tr>
<td>3.2</td>
<td>32x16</td>
<td>▘</td>
<td>❌</td>
</tr>

<tr>
<td rowspan="4">128x64 (hires)<br>SCHIP, XO-CHIP</td>
<td rowspan="3">1.0.0</td>
<td>256x64</td>
<td>██</td>
<td>✔</td>
</tr>

<tr>
<td>128x64</td>
<td>█</td>
<td>✔</td>
</tr>

<tr>
<td>128x32</td>
<td>▀</td>
<td>✔</td>
</tr>

<tr>
<td>3.2</td>
<td>64x32</td>
<td>▘</td>
<td>❌</td>
</tr>

</table><table>
<tr>
<th>CHIP-8 resolution</td>
<th>Unicode version</td>
<th>Minimum terminal size</td>
<th>Pixel size</td>
<th>XO-CHIP colors</td>
</tr>

<tr>
<td rowspan="5">64x32 (lores)<br>CHIP-8, SCHIP, XO-CHIP</td>
<td rowspan="3">1.0.0</td>
<td>128x32</td>
<td>██</td>
<td>✔</td>
</tr>

<tr>
<td>64x32</td>
<td>█</td>
<td>✔</td>
</tr>

<tr>
<td>64x16</td>
<td>▀</td>
<td>✔</td>
</tr>

<tr>
<td>3.2</td>
<td>32x16</td>
<td>▘</td>
<td>❌</td>
</tr>

<tr>
<td>3.0</td>
<td>32x8</td>
<td>⠁</td>
<td>❌</td>
</tr>

<tr>
<td rowspan="5">128x64 (hires)<br>SCHIP, XO-CHIP</td>
<td rowspan="3">1.0.0</td>
<td>256x64</td>
<td>██</td>
<td>✔</td>
</tr>

<tr>
<td>128x64</td>
<td>█</td>
<td>✔</td>
</tr>

<tr>
<td>128x32</td>
<td>▀</td>
<td>✔</td>
</tr>

<tr>
<td>3.2</td>
<td>64x32</td>
<td>▘</td>
<td>❌</td>
</tr>

<tr>
<td>3.0</td>
<td>64x16</td>
<td>⠁</td>
<td>❌</td>
</tr>

</table>

Notes:
* In your browser, the smallest pixel block (▘) probably looks square, but this might not be the case with your monospace terminal font.
* Pretty much all fonts support the basic [Unicode Block Elements](https://en.wikipedia.org/wiki/Block_Elements) in Unicode 1.0.0 which are used for the larger pixel blocks (█, ▀ and ▄), but support for the smallest blocks (like ▘) from Unicode 3.0 and 3.2 is much less common. Font families like _DejaVu_ and _Fira Code_ support them.
* The smallest pixel blocks (like ⠁) are Unicode Braille symbols, which aren't as common. Patched [Nerd Fonts](https://www.nerdfonts.com/) support them.

## Limitations

* Some games might not detect keypresses correctly. This is because [detecting when a key is released is very hard in a terminal](https://blog.robertelder.org/detect-keyup-event-linux-terminal/). Termin-8 does an approximation of keypress duration, but your OS's "key repeat" settings will influence how often it can poll for key presses.
* Your terminal's bell will sound when there's sound, but XO-CHIP music is not supported (as a terminal can't play sound on its own).

## Configuration file

Termin-8 will look for a file named `.octo.rc` in the user's home directory, which can be used to configure some useful settings. This file is also used by [C-Octo](https://github.com/JohnEarnest/c-octo#configuration-file).

You can also supply a configuration file with the `-c` command line option. This can be useful for setting some options for specific games – colors, to match the author's artistic vision, or "quirky" behavior, to make the game run correctly.

The file has a traditional `.INI` structure – empty lines or lines beginning with `#` are ignored, and anything else consists of a key and value separated by `=`. Meaningful keys are as follows:

- `core.tickrate`: number of CHIP-8 instructions to execute per 60hz frame.
- `core.max_rom`: the maximum number of bytes the compiler will permit when assembling a ROM.
- `core.font`: one of {`octo`, `vip`, `dream_6800`, `eti_660`, `schip`, `fish`} to select the built-in CHIP-8 font.

- `color.plane0`, `color.plane1`, `color.plane2`, `color.plane3`: colors for the 4 XO-CHIP "plane" colors.
- `color.background`: the border drawn behind the CHIP-8 display when no sound is being played.
- `color.sound`: the alternate border color when sound is being played.

- `quirks.shift`: if `1`, `vx <<= vy` and `vx >>= vy` modify `vx` in place and ignore `vy`, like SCHIP.
- `quirks.loadstore`: if `1`, `load` and `store` do not post-increment `i`, like SCHIP.
- `quirks.jump0`: if `1`, emulate a buggy behavior of SCHIP on the HP-48: the 4 high bits of the target address of `jump0` determines the offset register used (instead of always `v0`).
- `quirks.logic`: if `1`, clear `vf` after `&=`,`|=` and `^=`. On the VIP, these instructions leave `vf` in an unknown state.
- `quirks.clip`: if `1`, do not "wrap" sprite drawing around the edges of the display.
- `quirks.vblank`: if `1`, drawing a sprite will block until the end of the 60hz frame, like the VIP.

All colors are specified as 6-digit RGB in hexadecimal, like `996600`. The default quirks settings, palette, and other options correspond to those of web-octo.

