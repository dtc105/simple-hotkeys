# Simple Hotkeys

A bare bones copy of [AutoHotkey](https://www.autohotkey.com/) for Hyprland.


## Installation

Install simple-hotkeys with cargo, make sure ~/.cargo/bin is in the path and you are in the `input` group.  If you just added yourself to the `input` group make sure to logout and sign back in.

```bash
  export PATH=$PATH:/home/$USER/.cargo/bin  # Add to path
  sudo usermod -aG input $USER              # Join the input group

  cargo install simple-hotkeys
```
    
## Usage

```bash
Usage: simple-hotkeys [OPTIONS]... FILE_PATH
Run a simple hotkey script

Options:
    -d, --debug         Turn on debug mode
    -h, --help          Display this message
```

## Documentation

Each line is split into 3 parts `<operation> <action> [modifier]`

### Operations

- on - Defines the trigger event for the script
- event - Defines an event to send
- sleep - Defines a delay between events

### Actions
#### For the `on` and `event` operations
- `key:code` - Sends a key event with a certain code, see [Key Codes](#key-codes)
- `mouse:code` - Sends a mouse event with a certain code, see [Mouse Codes](#mouse-codes)

#### For the `sleep` operation
- `duration` - How long to sleep for in ms

### Modifiers
#### For the `on` operation
- `once` | `repeat` - Should the script repeat if the trigger is still pressed, Default: `once`

#### For the `event` operation
- `press` | `release` | `click` - The direction of the event, Default: `click`

## Examples

```text
# example.shk
on mouse:1 repeat

sleep 1000
event key:c down
sleep 500
event key:o
event key:o
event key:l
event key:c up
sleep 30
event mouse:2
```

This script waits until mouse 1 is pressed and will then wait a second before pressing 'c', waiting half a second, then typing 'ool' and releasing 'c', then waiting 30 ms and then right clicking.  It will repeat if the mouse 1 button is still down.


## Codes

<a id="key-codes"></a>
### Key Codes
The list of events is less complete than the trigger codes.  Keys that are unsupported for the events will be marked with an *.  There may (are) some inconsistencies with this list to the actual values, for the true list see `src/parser.rs` and look for the `parse_xxx_key_string` functions.  All patterns are case insensitive. (Don't ask me what half of these are)

| Key Name | Patterns | Code |
|----------|----------|------|
| Escape | "escape", "esc" | 1 |
| 1 | "1" | 2 |
| 2 | "2" | 3 |
| 3 | "3" | 4 |
| 4 | "4" | 5 |
| 5 | "5" | 6 |
| 6 | "6" | 7 |
| 7 | "7" | 8 |
| 8 | "8" | 9 |
| 9 | "9" | 10 |
| 0 | "0" | 11 |
| Minus | "minus", "-" | 12 |
| Equal | "equal", "=" | 13 |
| Backspace | "backspace" | 14 |
| Tab | "tab" | 15 |
| Q | "q" | 16 |
| W | "w" | 17 |
| E | "e" | 18 |
| R | "r" | 19 |
| T | "t" | 20 |
| Y | "y" | 21 |
| U | "u" | 22 |
| I | "i" | 23 |
| O | "o" | 24 |
| P | "p" | 25 |
| Left Brace | "leftbrace", "lbrace", "[" | 26 |
| Right Brace | "rightbrace", "rbrace", "]" | 27 |
| Return | "return", "ret", "enter" | 28 |
| Left Control | "leftcontrol", "lcontrol", "leftctrl", "lctrl" | 29 |
| A | "a" | 30 |
| S | "s" | 31 |
| D | "d" | 32 |
| F | "f" | 33 |
| G | "g" | 34 |
| H | "h" | 35 |
| J | "j" | 36 |
| K | "k" | 37 |
| L | "l" | 38 |
| Semicolon | "semicolon", ";" | 39 |
| Apostrophe | "apostrophe", "'" | 40 |
| *Grave | "grave", "tilde", "~" | 41 |
| Left Shift | "leftshift", "lshift" | 42 |
| Backslash | "backslash", "bslash", "\\" | 43 |
| Z | "z" | 44 |
| X | "x" | 45 |
| C | "c" | 46 |
| V | "v" | 47 |
| B | "b" | 48 |
| N | "n" | 49 |
| M | "m" | 50 |
| Comma | "comma", "," | 51 |
| Decimal | "decimal", "period", "dot", "." | 52 |
| Forward Slash | "forwardslash", "fslash", "slash", "/" | 53 |
| Right Shift | "rightshift", "rshift" | 54 |
| Numpad Asterisk | "numasterisk", "asterisk", "*" | 55 |
| Left Alt | "leftalt", "lalt" | 56 |
| Space | "space" | 57 |
| Caps Lock | "capslock", "caps" | 58 |
| F1 | "f1" | 59 |
| F2 | "f2" | 60 |
| F3 | "f3" | 61 |
| F4 | "f4" | 62 |
| F5 | "f5" | 63 |
| F6 | "f6" | 64 |
| F7 | "f7" | 65 |
| F8 | "f8" | 66 |
| F9 | "f9" | 67 |
| F10 | "f10" | 68 |
| Numpad Lock | "numlock" | 69 |
| Scroll Lock | "scrolllock" | 70 |
| Numpad Seven | "numseven", "num7" | 71 |
| Numpad Eight | "numeight", "num8" | 72 |
| Numpad Nine | "numnine", "num9" | 73 |
| Numpad Minus | "numminus", "num-" | 74 |
| Numpad Four | "numfour", "num4" | 75 |
| Numpad Five | "numfive", "num5" | 76 |
| Numpad Six | "numsix", "num6" | 77 |
| Numpad Add | "numadd", "num+" | 78 |
| Numpad One | "numone", "num1" | 79 |
| Numpad Two | "numtwo", "num2" | 80 |
| Numpad Three | "numthree", "num3" | 81 |
| Numpad Zero | "numzero", "num0" | 82 |
| Numpad Dot | "numdot", "num." | 83 |
| *Zenkaku Kanakaku | "zenkakukanakaku" | 85 |
| *102nd | "102nd" | 86 |
| F11 | "f11" | 87 |
| F12 | "f12" | 88 |
| *Ro | "ro" | 89 |
| *Katakana | "katakana" | 90 |
| *Hiragana | "hiragana" | 91 |
| *Henkan | "henkan" | 92 |
| *Katakana Hiragana | "katakanahiragana" | 93 |
| *Muhenkan | "muhenkan" | 94 |
| *Numpad Japanese Comma | "numjapanesecomma", "numjcomma", "numj," | 95 |
| *Numpad Enter | "numenter", "numreturn", "numret" | 96 |
| Right Control | "rightcontrol", "rcontrol", "rightctrl", "rctrl" | 97 |
| *Numpad Slash | "numslash", "num/" | 98 |
| System Request | "systemrequest", "sysrq" | 99 |
| Right Alt | "rightalt", "ralt" | 100 |
| Home | "home" | 102 |
| Up | "up" | 103 |
| Page Up | "pageup", "pageu" | 104 |
| Left | "left" | 105 |
| Right | "right" | 106 |
| End | "end" | 107 |
| Down | "down" | 108 |
| Page Down | "pagedown", "pagedn", "paged" | 109 |
| Insert | "insert", "ins" | 110 |
| Delete | "delete", "del" | 111 |
| Mute | "mute" | 113 |
| Volume Down | "volumedown", "volumedn", "volumed", "voldown", "voldn", "vold" | 114 |
| Volume Up | "volumeup", "volumeu", "volup", "volu" | 115 |
| Power | "power" | 116 |
| *Numpad Equal | "numequal", "num=" | 117 |
| Pause | "pause" | 119 |
| Numpad Comma | "numcomma", "num," | 121 |
| Hanguel | "hanguel" | 122 |
| Hanja | "hanja" | 123 |
| *Yen | "yen" | 124 |
| Left Meta | "leftmeta", "lmeta", "meta", "windows", "window", "win", "super", "command", "cmd" | 125 |
| Right Meta | "rightmeta", "rmeta" | 126 |
| *Compose | "compose" | 127 |
| Stop | "stop" | 128 |
| *Again | "again" | 129 |
| *Props | "props" | 130 |
| Undo | "undo" | 131 |
| *Front | "front" | 132 |
| Copy | "copy" | 133 |
| Open | "open" | 134 |
| Paste | "paste" | 135 |
| Find | "find" | 136 |
| Cut | "cut" | 137 |
| *Help | "help" | 138 |
| *Calc | "calc" | 140 |
| *Sleep | "sleep" | 142 |
| *WWW | "www" | 150 |
| Screen Lock | "screenlock" | 152 |
| Back | "back" | 158 |
| Forward | "forward" | 159 |
| Eject CD | "ejectcd", "eject" | 161 |
| Next Song | "nextsong", "next" | 163 |
| Play Pause | "playpause" | 164 |
| Previous Song | "previoussong", "prevsong", "previous", "prev" | 165 |
| Stop CD | "stopcd" | 166 |
| Refresh | "refresh" | 173 |
| Edit | "edit" | 176 |
| Scroll Up | "scrollup" | 177 |
| Scroll Down | "scrolldown" | 178 |
| *Numpad Left Parenthesis | "numleftparenthesis", "numlparenthesis", "num(" | 179 |
| *Numpad Right Parenthesis | "numrightparenthesis", "numrparenthesis", "num)" | 180 |
| F13 | "f13" | 183 |
| F14 | "f14" | 184 |
| F15 | "f15" | 185 |
| F16 | "f16" | 186 |
| F17 | "f17" | 187 |
| F18 | "f18" | 188 |
| F19 | "f19" | 189 |
| F20 | "f20" | 190 |
| F21 | "f21" | 191 |
| F22 | "f22" | 192 |
| F23 | "f23" | 193 |
| F24 | "f24" | 194 |

<a id="mouse-codes"></a>
### Mouse Codes
For the mouse codes the only option is to put the number corresponding to the button. For example the left mouse button is code 1 so in the .shk file I would put `mouse:1`.

| Key Name | Code |
| -------- | ---- |
| Left | 1 |
| Right | 2 |
| Middle | 3 |
| Back | 4 |
| Forward | 5 |


## Support

This has only ever been tested on Arch Linux 6.16.5+ with Hyprland.  If you have an issue running on your system then good luck.


## Contributing

If you want to contribute please open a pull request or issue.


