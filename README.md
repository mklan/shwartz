# shwartz

An overlay to black out pixels on an OLED display. The goal is to reduce energy consumption. Can be of course also used on other panels, but with less effect.

It is cross-platform but only tested on wayland.

![Preview](https://github.com/mklan/shwartz/blob/main/preview.png?raw=true)

## Build

`cargo build --release`

## Usage

```bash
Usage: shwartz [OPTIONS]

Options:
  -w, --width <WIDTH>    Width of the screen [default: 1920]
  -h, --height <HEIGHT>  Height of the screen [default: 1080]
  -e, --every <EVERY>    How often a pixel should be blacked out [default: 2]
  -c, --color <COLOR>    Overwrite color [default: 0xFF000000]
```

Furthermore you need to tell your window manager how to handle shwartz windows. This means going fullscreen, on top, pinned on every desktop, unfocused to pass input and on which monitor.


### Hyprland example

Full overlay

```bash
hyprctl dispatch exec "[float;pin;nofocus;noblur;noborder;nodim;noanim;fakefullscreen]" /path/to/bin/shwartz
```

Kill it on demand with

`pkill -f shwartz`

Cover only a part (for example a bar or dock)

```bash
hyprctl dispatch exec "[float;pin;nofocus;noblur;noborder;nodim;noanim;fakefullscreen;move 0 0]" "/path/to/bin/shwartz --height 30 --every 3"
```