Termin-8
========
[![crates.io](https://img.shields.io/crates/v/termin-8.svg)](https://crates.io/crates/termin-8)

Octo-compliant CHIP-8 emulator frontend that runs in your terminal.

It uses [`deca`](https://crates.io/crates/deca) as the emulator backend, which supports CHIP-8, SUPER-CHIP (SCHIP) and XO-CHIP programs.

Note that the terminal size requirements vary depending on what kind of program you attempt to run.

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
<td rowspan="4">64x32 (lores)<br>CHIP-8, SCHIP, XO-CHIP</td>
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

</table>

Notes:
* In your browser, the smallest pixel block (▘) probably looks square, but this might not be the case with your monospace terminal font.
* Pretty much all fonts support the basic [Unicode Block Elements](https://en.wikipedia.org/wiki/Block_Elements) in Unicode 1.0.0 which are used for the larger pixel blocks (█, ▀ and ▄), but support for the smallest blocks (like ▘) from Unicode 3.2 is much less common. Font families like _DejaVu_ and _Fira Code_ support them.

## Installation

```sh
cargo install termin-8
```

## Usage

```sh
termin-8 ROM
``` 
