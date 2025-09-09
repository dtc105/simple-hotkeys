use std::time::Duration;

use crate::parser::{Action, Script, Trigger};

extern crate libc;
use enigo::{Button, Enigo, Keyboard, Mouse, Settings};
use input::{
    Event, Libinput, LibinputInterface,
    event::{
        PointerEvent,
        keyboard::{KeyState, KeyboardEventTrait},
        pointer::ButtonState,
    },
};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

use libc::{O_RDONLY, O_RDWR, O_WRONLY};

#[derive(Default)]
pub struct Options {
    debug: bool,
}

pub struct Runner {
    input: Option<Libinput>,
    script: Script,
    options: Options,
}

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn is_trigger(trigger: &Trigger, event: &Event) -> Option<bool> {
    match (trigger, event) {
        (Trigger::Key(trigger_code), Event::Keyboard(event)) => {
            if &event.key() != trigger_code {
                return None;
            }

            Some(matches!(event.key_state(), KeyState::Pressed))
        }
        (Trigger::Mouse(trigger_code), Event::Pointer(PointerEvent::Button(button))) => {
            if &(button.button() - 271) != trigger_code {
                return None;
            }

            Some(matches!(&button.button_state(), ButtonState::Pressed))
        }
        _ => None,
    }
}

fn button_from_u16(x: u16) -> Option<Button> {
    match x {
        1 => Some(Button::Left),
        2 => Some(Button::Right),
        3 => Some(Button::Middle),
        4 => Some(Button::Back),
        5 => Some(Button::Forward),
        _ => None,
    }
}

impl Runner {
    pub fn new() -> Self {
        let mut script_path: Option<String> = None;
        let mut options = Options::default();
        let mut args = std::env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-d" | "--debug" => options.debug = true,
                arg => {
                    if arg.starts_with('-') {
                        panic!("Unknown argument: {arg}");
                    } else {
                        if script_path.is_none() {
                            script_path = Some(arg.to_string());
                        } else {
                            panic!("Script path already set as '{}'", script_path.unwrap());
                        }
                    }
                }
            }
        }

        let script = Script::read(script_path.expect("No script path set."));

        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();

        Self {
            input: Some(input),
            script,
            options,
        }
    }

    fn execute_actions(&self, enigo: &mut Enigo) {
        if self.options.debug {
            println!("Trigger received!");
        }

        for action in &self.script.actions {
            if self.options.debug {
                println!("Action: {:?}", action);
            }
            match action {
                Action::KeyEvent { key, direction } => enigo
                    .key(*key, *direction)
                    .expect("Could not send key event."),
                Action::MouseEvent { code, direction } => enigo
                    .button(button_from_u16(*code).unwrap(), *direction)
                    .expect("Could not send mouse event."),
                Action::Sleep(duration) => {
                    std::thread::sleep(Duration::from_millis(*duration));
                }
            };
        }
    }

    pub fn run(&mut self) {
        let settings = Settings {
            linux_delay: 10,
            x11_display: None,     //Some(":0".to_string()),
            wayland_display: None, //Some("wayland-1".to_string()),
            windows_dw_extra_info: None,
            event_source_user_data: None,
            release_keys_when_dropped: true,
            open_prompt_to_get_permissions: true,
            independent_of_keyboard_state: true,
            windows_subject_to_mouse_speed_and_acceleration_level: false,
        };

        let mut enigo = Enigo::new(&settings).expect("Could not initialize enigo.");
        let mut trigger_down: bool = false;
        let mut input = std::mem::take(&mut self.input).unwrap();

        if self.script.is_repeating {
            loop {
                input.dispatch().unwrap();

                for event in &mut input {
                    if let Some(is_pressed) = is_trigger(&self.script.trigger, &event) {
                        if trigger_down ^ is_pressed {
                            trigger_down = is_pressed;
                            break;
                        }
                    }
                }

                if trigger_down {
                    std::thread::sleep(Duration::from_millis(10));
                    self.execute_actions(&mut enigo);
                }
            }
        } else {
            loop {
                input.dispatch().unwrap();
                for event in &mut input {
                    if is_trigger(&self.script.trigger, &event).unwrap_or(false) {
                        std::thread::sleep(Duration::from_millis(10));
                        self.execute_actions(&mut enigo);
                        break;
                    }
                }
            }
        }
    }
}
