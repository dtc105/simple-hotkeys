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
    pub trigger: Trigger,
    pub actions: Vec<Action>,
}

fn parse_trigger_key_string(key_str: &str) -> Option<u16> {
    match key_str.to_lowercase().as_str() {
        "lshift" | "leftshift" => Some(42),  // KEY_LEFTSHIFT
        "rshift" | "rightshift" => Some(54), // KEY_RIGHTSHIFT
        "lctrl" | "leftctrl" => Some(29),    // KEY_LEFTCTRL
        "rctrl" | "rightctrl" => Some(97),   // KEY_RIGHTCTRL
        "lalt" | "leftalt" => Some(56),      // KEY_LEFTALT
        "ralt" | "rightalt" => Some(100),    // KEY_RIGHTALT
        "tab" => Some(15),                   // KEY_TAB
        "enter" => Some(28),                 // KEY_ENTER
        "space" => Some(57),                 // KEY_SPACE
        "esc" | "escape" => Some(1),         // KEY_ESC
        "backspace" => Some(14),             // KEY_BACKSPACE
        "del" | "delete" => Some(46),        // KEY_DELETE
        "a" => Some(30),                     // KEY_A
        "b" => Some(48),                     // KEY_B
        "c" => Some(46),                     // KEY_C
        "d" => Some(32),                     // KEY_D
        "e" => Some(18),                     // KEY_E
        "f" => Some(33),                     // KEY_F
        "g" => Some(34),                     // KEY_G
        "h" => Some(35),                     // KEY_H
        "i" => Some(23),                     // KEY_I
        "j" => Some(36),                     // KEY_J
        "k" => Some(37),                     // KEY_K
        "l" => Some(38),                     // KEY_L
        "m" => Some(50),                     // KEY_M
        "n" => Some(49),                     // KEY_N
        "o" => Some(24),                     // KEY_O
        "p" => Some(25),                     // KEY_P
        "q" => Some(16),                     // KEY_Q
        "r" => Some(19),                     // KEY_R
        "s" => Some(31),                     // KEY_S
        "t" => Some(20),                     // KEY_T
        "u" => Some(22),                     // KEY_U
        "v" => Some(47),                     // KEY_V
        "w" => Some(17),                     // KEY_W
        "x" => Some(45),                     // KEY_X
        "y" => Some(21),                     // KEY_Y
        "z" => Some(44),                     // KEY_Z
        "0" => Some(11),                     // KEY_0
        "1" => Some(2),                      // KEY_1
        "2" => Some(3),                      // KEY_2
        "3" => Some(4),                      // KEY_3
        "4" => Some(5),                      // KEY_4
        "5" => Some(6),                      // KEY_5
        "6" => Some(7),                      // KEY_6
        "7" => Some(8),                      // KEY_7
        "8" => Some(9),                      // KEY_8
        "9" => Some(10),                     // KEY_9
        "f1" => Some(59),                    // KEY_F1
        "f2" => Some(60),                    // KEY_F2
        "f3" => Some(61),                    // KEY_F3
        "f4" => Some(62),                    // KEY_F4
        "f5" => Some(63),                    // KEY_F5
        "f6" => Some(64),                    // KEY_F6
        "f7" => Some(65),                    // KEY_F7
        "f8" => Some(66),                    // KEY_F8
        "f9" => Some(67),                    // KEY_F9
        "f10" => Some(68),                   // KEY_F10
        "f11" => Some(87),                   // KEY_F11
        "f12" => Some(88),                   // KEY_F12
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
        "clear" => Some(Key::Clear),
        "command" | "cmd" | "super" | "windows" | "win" => Some(Key::Meta),
        "control" | "ctrl" => Some(Key::Control),
        "decimal" => Some(Key::Decimal),
        "delete" | "del" => Some(Key::Delete),
        "divide" => Some(Key::Divide),
        "downarrow" | "down" => Some(Key::DownArrow),
        "end" => Some(Key::End),
        "escape" | "esc" => Some(Key::Escape),
        "execute" => Some(Key::Execute),
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
        "lcontrol" => Some(Key::LControl),
        "leftarrow" | "left" => Some(Key::LeftArrow),
        "linefeed" => Some(Key::Linefeed),
        "lmenu" => Some(Key::LMenu),
        "lshift" => Some(Key::LShift),
        "medianexttrack" | "nexttrack" | "next" => Some(Key::MediaNextTrack),
        "mediaplaypause" | "playpause" | "play" => Some(Key::MediaPlayPause),
        "mediaprevtrack" | "prevtrack" | "prev" => Some(Key::MediaPrevTrack),
        "mediastop" | "stop" => Some(Key::MediaStop),
        "meta" => Some(Key::Meta),
        "modechange" => Some(Key::ModeChange),
        "multiply" => Some(Key::Multiply),
        "numlock" => Some(Key::Numlock),
        "numpad0" => Some(Key::Numpad0),
        "numpad1" => Some(Key::Numpad1),
        "numpad2" => Some(Key::Numpad2),
        "numpad3" => Some(Key::Numpad3),
        "numpad4" => Some(Key::Numpad4),
        "numpad5" => Some(Key::Numpad5),
        "numpad6" => Some(Key::Numpad6),
        "numpad7" => Some(Key::Numpad7),
        "numpad8" => Some(Key::Numpad8),
        "numpad9" => Some(Key::Numpad9),
        "option" => Some(Key::Option),
        "pagedown" | "pgdn" => Some(Key::PageDown),
        "pageup" | "pgup" => Some(Key::PageUp),
        "pause" => Some(Key::Pause),
        "printscr" | "prtsc" | "printscreen" | "print" => Some(Key::PrintScr),
        "rcontrol" => Some(Key::RControl),
        "redo" => Some(Key::Redo),
        "return" | "enter" => Some(Key::Return),
        "rightarrow" | "right" => Some(Key::RightArrow),
        "rshift" => Some(Key::RShift),
        "scrolllock" => Some(Key::ScrollLock),
        "select" => Some(Key::Select),
        "scriptswitch" => Some(Key::ScriptSwitch),
        "shift" => Some(Key::Shift),
        "shiftlock" => Some(Key::ShiftLock),
        "space" => Some(Key::Space),
        "subtract" => Some(Key::Subtract),
        "sysreq" => Some(Key::SysReq),
        "tab" => Some(Key::Tab),
        "undo" => Some(Key::Undo),
        "uparrow" | "up" => Some(Key::UpArrow),
        "volumedown" | "voldown" => Some(Key::VolumeDown),
        "volumemute" | "mute" => Some(Key::VolumeMute),
        "volumeup" | "volup" => Some(Key::VolumeUp),
        "micmute" => Some(Key::MicMute),
        _ => None,
    }
}

fn parse_direction(dir_str: &str) -> Option<Direction> {
    match dir_str.to_lowercase().as_str() {
        "d" | "down" | "p" | "press" => Some(Direction::Press),
        "u" | "up" | "r" | "release" => Some(Direction::Release),
        "c" | "click" => Some(Direction::Click),
        _ => None,
    }
}

impl Script {
    pub fn read(script_path: String) -> Self {
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

        if trigger.is_none() {
            panic!("No trigger set.");
        }

        Self {
            trigger: trigger.unwrap(),
            actions,
        }
    }
}

impl IntoIterator for Script {
    type Item = Action;
    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}
