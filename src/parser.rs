use enigo::{Direction, Key};

#[derive(Debug)]
pub enum Action {
    KeyEvent { key: Key, direction: Direction },
    MouseEvent { code: u16, direction: Direction },
    Sleep(u64),
}

pub enum Trigger {
    Mouse(u32),
    Key(u32),
}

pub struct Script {
    pub is_repeating: bool,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
}

fn parse_trigger_key_string(key_str: &str) -> Option<u16> {
    match key_str.to_lowercase().as_str() {
        "escape" | "esc" => Some(1),
        "1" => Some(2),
        "2" => Some(3),
        "3" => Some(4),
        "4" => Some(5),
        "5" => Some(6),
        "6" => Some(7),
        "7" => Some(8),
        "8" => Some(9),
        "9" => Some(10),
        "0" => Some(11),
        "minus" | "-" => Some(12),
        "equal" | "=" => Some(13),
        "backspace" => Some(14),
        "tab" => Some(15),
        "q" => Some(16),
        "w" => Some(17),
        "e" => Some(18),
        "r" => Some(19),
        "t" => Some(20),
        "y" => Some(21),
        "u" => Some(22),
        "i" => Some(23),
        "o" => Some(24),
        "p" => Some(25),
        "leftbrace" | "lbrace" | "[" => Some(26),
        "rightbrace" | "rbrace" | "]" => Some(27),
        "return" | "ret" | "enter" => Some(28),
        "leftcontrol" | "lcontrol" | "leftctrl" | "lctrl" => Some(29),
        "a" => Some(30),
        "s" => Some(31),
        "d" => Some(32),
        "f" => Some(33),
        "g" => Some(34),
        "h" => Some(35),
        "j" => Some(36),
        "k" => Some(37),
        "l" => Some(38),
        "semicolon" | ";" => Some(39),
        "apostrophe" | "'" => Some(40),
        "grave" | "tilde" | "~" => Some(41),
        "leftshift" | "lshift" => Some(42),
        "backslash" | "bslash" | "\\" => Some(43),
        "z" => Some(44),
        "x" => Some(45),
        "c" => Some(46),
        "v" => Some(47),
        "b" => Some(48),
        "n" => Some(49),
        "m" => Some(50),
        "comma" | "," => Some(51),
        "decimal" | "period" | "dot" | "." => Some(52),
        "forwardslash" | "fslash" | "slash" | "/" => Some(53),
        "rightshift" | "rshift" => Some(54),
        "numasterisk" | "asterisk" | "num*" => Some(55),
        "leftalt" | "lalt" => Some(56),
        "space" => Some(57),
        "capslock" | "caps" => Some(58),
        "f1" => Some(59),
        "f2" => Some(60),
        "f3" => Some(61),
        "f4" => Some(62),
        "f5" => Some(63),
        "f6" => Some(64),
        "f7" => Some(65),
        "f8" => Some(66),
        "f9" => Some(67),
        "f10" => Some(68),
        "numlock" => Some(69),
        "scrolllock" => Some(70),
        "numseven" | "num7" => Some(71),
        "numeight" | "num8" => Some(72),
        "numnine" | "num9" => Some(73),
        "numminus" | "num-" => Some(74),
        "numfour" | "num4" => Some(75),
        "numfive" | "num5" => Some(76),
        "numsix" | "num6" => Some(77),
        "numadd" | "num+" => Some(78),
        "numone" | "num1" => Some(79),
        "numtwo" | "num2" => Some(80),
        "numthree" | "num3" => Some(81),
        "numzero" | "num0" => Some(82),
        "numdot" | "num." => Some(83),
        "zenkakukanakaku" => Some(85),
        "102nd" => Some(86),
        "f11" => Some(87),
        "f12" => Some(88),
        "ro" => Some(89),
        "katakana" => Some(90),
        "hiragana" => Some(91),
        "henkan" => Some(92),
        "katakanahiragana" => Some(93),
        "muhenkan" => Some(94),
        "numjapanesecomma" | "numjcomma" | "numj," => Some(95),
        "numenter" | "numreturn" | "numret" => Some(96),
        "rightcontrol" | "rcontrol" | "rightctrl" | "rctrl" => Some(97),
        "numslash" | "num/" => Some(98),
        "systemrequest" | "sysrq" => Some(99),
        "rightalt" | "ralt" => Some(100),
        "home" => Some(102),
        "up" => Some(103),
        "pageup" | "pageu" => Some(104),
        "left" => Some(105),
        "right" => Some(106),
        "end" => Some(107),
        "down" => Some(108),
        "pagedown" | "pagedn" | "paged" => Some(109),
        "insert" | "ins" => Some(110),
        "delete" | "del" => Some(111),
        "mute" => Some(113),
        "volumedown" | "volumedn" | "volumed" | "voldown" | "voldn" | "vold" => Some(114),
        "volumeup" | "volumeu" | "volup" | "volu" => Some(115),
        "power" => Some(116),
        "numequal" | "num=" => Some(117),
        "pause" => Some(119),
        "numcomma" | "num," => Some(121),
        "hanguel" => Some(122),
        "hanja" => Some(123),
        "yen" => Some(124),
        "leftmeta" | "lmeta" | "meta" | "windows" | "window" | "win" | "super" | "command"
        | "cmd" => Some(125),
        "rightmeta" | "rmeta" => Some(126),
        "compose" => Some(127),
        "stop" => Some(128),
        "again" => Some(129),
        "props" => Some(130),
        "undo" => Some(131),
        "front" => Some(132),
        "copy" => Some(133),
        "open" => Some(134),
        "paste" => Some(135),
        "find" => Some(136),
        "cut" => Some(137),
        "help" => Some(138),
        "calc" => Some(140),
        "sleep" => Some(142),
        "www" => Some(150),
        "screenlock" => Some(152),
        "back" => Some(158),
        "forward" => Some(159),
        "ejectcd" | "eject" => Some(161),
        "nextsong" | "next" => Some(163),
        "playpause" => Some(164),
        "previoussong" | "prevsong" | "previous" | "prev" => Some(165),
        "stopcd" => Some(166),
        "refresh" => Some(173),
        "edit" => Some(176),
        "scrollup" => Some(177),
        "scrolldown" => Some(178),
        "numleftparenthesis" | "numlparenthesis" | "num(" => Some(179),
        "numrightparenthesis" | "numrparenthesis" | "num)" => Some(180),
        "f13" => Some(183),
        "f14" => Some(184),
        "f15" => Some(185),
        "f16" => Some(186),
        "f17" => Some(187),
        "f18" => Some(188),
        "f19" => Some(189),
        "f20" => Some(190),
        "f21" => Some(191),
        "f22" => Some(192),
        "f23" => Some(193),
        "f24" => Some(194),
        _ => None,
    }
}

fn parse_action_key_string(key_str: &str) -> Option<Key> {
    if key_str.len() == 1 {
        return Some(Key::Unicode(key_str.to_lowercase().chars().next().unwrap()));
    }

    match key_str.to_lowercase().as_str() {
        "add" => Some(Key::Add),
        "alt" => Some(Key::Alt),
        "backspace" => Some(Key::Backspace),
        "break" => Some(Key::Break),
        "begin" => Some(Key::Begin),
        "cancel" => Some(Key::Cancel),
        "capslock" => Some(Key::CapsLock),
        "clear" | "clr" => Some(Key::Clear),
        "command" | "cmd" | "super" | "windows" | "win" | "meta" => Some(Key::Meta),
        "control" | "ctrl" => Some(Key::Control),
        "decimal" | "period" | "dot" => Some(Key::Decimal),
        "delete" | "del" => Some(Key::Delete),
        "divide" => Some(Key::Divide),
        "downarrow" | "down" => Some(Key::DownArrow),
        "end" => Some(Key::End),
        "escape" | "esc" => Some(Key::Escape),
        "execute" | "exec" => Some(Key::Execute),
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),
        "f13" => Some(Key::F13),
        "f14" => Some(Key::F14),
        "f15" => Some(Key::F15),
        "f16" => Some(Key::F16),
        "f17" => Some(Key::F17),
        "f18" => Some(Key::F18),
        "f19" => Some(Key::F19),
        "f20" => Some(Key::F20),
        "f21" => Some(Key::F21),
        "f22" => Some(Key::F22),
        "f23" => Some(Key::F23),
        "f24" => Some(Key::F24),
        "f25" => Some(Key::F25),
        "f26" => Some(Key::F26),
        "f27" => Some(Key::F27),
        "f28" => Some(Key::F28),
        "f29" => Some(Key::F29),
        "f30" => Some(Key::F30),
        "f31" => Some(Key::F31),
        "f32" => Some(Key::F32),
        "f33" => Some(Key::F33),
        "f34" => Some(Key::F34),
        "f35" => Some(Key::F35),
        "find" => Some(Key::Find),
        "hangul" => Some(Key::Hangul),
        "hanja" => Some(Key::Hanja),
        "help" => Some(Key::Help),
        "home" => Some(Key::Home),
        "insert" | "ins" => Some(Key::Insert),
        "kanji" => Some(Key::Kanji),
        "leftcontrol" | "lcontrol" | "leftctrl" | "lctrl" => Some(Key::LControl),
        "leftarrow" | "left" => Some(Key::LeftArrow),
        "linefeed" => Some(Key::Linefeed),
        "leftmenu" | "lmenu" => Some(Key::LMenu),
        "leftshift" | "lshift" => Some(Key::LShift),
        "medianexttrack" | "nexttrack" | "next" => Some(Key::MediaNextTrack),
        "mediaplaypause" | "playpause" | "play" => Some(Key::MediaPlayPause),
        "mediaprevtrack" | "prevtrack" | "prev" => Some(Key::MediaPrevTrack),
        "mediastop" | "stop" => Some(Key::MediaStop),
        "modechange" => Some(Key::ModeChange),
        "multiply" | "mult" | "mul" | "times" => Some(Key::Multiply),
        "numlock" => Some(Key::Numlock),
        "numzero" | "num0" => Some(Key::Numpad0),
        "numone" | "num1" => Some(Key::Numpad1),
        "numtwo" | "num2" => Some(Key::Numpad2),
        "numthree" | "num3" => Some(Key::Numpad3),
        "numfour" | "num4" => Some(Key::Numpad4),
        "numfive" | "num5" => Some(Key::Numpad5),
        "numsix" | "num6" => Some(Key::Numpad6),
        "numseven" | "num7" => Some(Key::Numpad7),
        "numeight" | "num8" => Some(Key::Numpad8),
        "numnine" | "num9" => Some(Key::Numpad9),
        "option" => Some(Key::Option),
        "pagedown" | "pgdn" => Some(Key::PageDown),
        "pageup" | "pgup" => Some(Key::PageUp),
        "pause" => Some(Key::Pause),
        "printscr" | "prtsc" | "printscreen" | "print" => Some(Key::PrintScr),
        "rightcontrol" | "rcontrol" | "rightctrl" | "rctrl" => Some(Key::RControl),
        "redo" => Some(Key::Redo),
        "return" | "ret" | "enter" => Some(Key::Return),
        "rightarrow" | "right" => Some(Key::RightArrow),
        "rightshift" | "rshift" => Some(Key::RShift),
        "scrolllock" => Some(Key::ScrollLock),
        "select" | "sel" => Some(Key::Select),
        "scriptswitch" => Some(Key::ScriptSwitch),
        "shift" => Some(Key::Shift),
        "shiftlock" => Some(Key::ShiftLock),
        "space" => Some(Key::Space),
        "subtract" | "sub" | "minus" => Some(Key::Subtract),
        "systemrequest" | "sysreq" => Some(Key::SysReq),
        "tab" => Some(Key::Tab),
        "undo" => Some(Key::Undo),
        "uparrow" | "up" => Some(Key::UpArrow),
        "volumedown" | "voldown" | "voldn" => Some(Key::VolumeDown),
        "volumemute" | "mute" => Some(Key::VolumeMute),
        "volumeup" | "volup" => Some(Key::VolumeUp),
        "micmute" => Some(Key::MicMute),
        _ => None,
    }
}

fn parse_direction(dir_str: &str) -> Option<Direction> {
    match dir_str.to_lowercase().as_str() {
        "down" | "d" | "press" | "p" => Some(Direction::Press),
        "up" | "u" | "release" | "r" => Some(Direction::Release),
        "click" | "c" => Some(Direction::Click),
        _ => None,
    }
}

impl Script {
    pub fn read(script_path: String) -> Self {
        let mut is_repeating = false;
        let mut trigger: Option<Trigger> = None;
        let mut actions: Vec<Action> = Vec::new();

        let file = std::fs::read_to_string(&script_path)
            .expect(format!("File not found: {}", script_path).as_str());
        let lines = file.lines().map(String::from).into_iter();

        let mut line_number = 0;

        for line in lines {
            line_number += 1;

            if line.len() == 0 || line.starts_with('#') {
                continue;
            }

            let mut words = line.split_whitespace();
            let operation = words
                .next()
                .expect(format!("No operation on line: {line_number}").as_str());

            match operation.to_lowercase().as_str() {
                "on" => {
                    let mut action_string = words
                        .next()
                        .expect(format!("No trigger passed. Line {line_number}").as_str())
                        .split(':');

                    let action_type = action_string
                        .next()
                        .expect(format!("No action type passed. Line {line_number}").as_str());
                    let action_value = action_string
                        .next()
                        .expect(format!("No action value passed. Line {line_number}").as_str());

                    trigger = Some(match action_type.to_lowercase().as_str() {
                        "key" | "k" => Trigger::Key(parse_trigger_key_string(action_value).expect(
                            format!("Could not parse action value. Line {line_number}").as_str(),
                        ) as u32),
                        "mouse" | "m" => Trigger::Mouse(action_value.parse::<u32>().expect(
                            format!("Could not parse action value. Line {line_number}").as_str(),
                        )),
                        _ => panic!("Could not parse action type. Line {line_number}"),
                    });

                    is_repeating = match words.next().unwrap_or("").to_lowercase().as_str() {
                        "repeating" | "repeat" | "r" => true,
                        _ => false,
                    };
                }
                "event" => {
                    let mut action_string = words
                        .next()
                        .expect(format!("No trigger passed. Line {line_number}").as_str())
                        .split(':');

                    let action_type = action_string
                        .next()
                        .expect(format!("No action type passed. Line {line_number}").as_str());
                    let action_value = action_string
                        .next()
                        .expect(format!("No action value passed. Line {line_number}").as_str());
                    let action_direction = words.next().unwrap_or("click");

                    match action_type.to_lowercase().as_str() {
                        "key" | "k" => {
                            let key = parse_action_key_string(action_value).expect(
                                format!("Could not parse action value. Line {line_number}")
                                    .as_str(),
                            );

                            let direction = parse_direction(action_direction).expect(
                                format!("Could not parse direction. Line {line_number}").as_str(),
                            );

                            let action = Action::KeyEvent { key, direction };

                            actions.push(action);
                        }
                        "mouse" | "m" => {
                            let code = action_value.parse::<u16>().expect(
                                format!("Could not parse action value: Line {line_number}")
                                    .as_str(),
                            );

                            let direction = parse_direction(action_direction).expect(
                                format!("Could not parse direction. Line {line_number}").as_str(),
                            );

                            let action = Action::MouseEvent { code, direction };

                            actions.push(action);
                        }
                        _ => panic!("Could not parse action type. Line {line_number}"),
                    }
                }
                "sleep" => {
                    let duration_string = words
                        .next()
                        .expect(format!("No duration passed. Line {line_number}").as_str());
                    let duration = duration_string
                        .parse::<u64>()
                        .expect(format!("Could not parse duration. Line {line_number}").as_str());

                    let action = Action::Sleep(duration);

                    actions.push(action);
                }
                _ => panic!("Could not parse operation. Line {line_number}"),
            }
        }

        Self {
            is_repeating,
            trigger: trigger.expect("No trigger set."),
            actions,
        }
    }
}
