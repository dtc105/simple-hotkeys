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

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap_or(-1))
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

pub struct Runner {
    script: Script,
}

impl Runner {
    pub fn new() -> Self {
        let mut script_path: Option<String> = None;
        let mut args = std::env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => Runner::display_help(),
                "-d" | "--debug" => unsafe {
                    std::env::set_var("RUST_LOG", "debug");
                },
                arg => {
                    if arg.starts_with('-') {
                        panic!("Unknown argument: {arg}");
                    } else {
                        if let Some(path) = script_path {
                            panic!("Script path already set as '{}'", path);
                        } else {
                            script_path = Some(arg.to_string());
                        }
                    }
                }
            }
        }

        let script = Script::read(script_path.expect("No script path set."));

        Self { script }
    }

    fn display_help() {
        println!(
            r#"Usage: simple-hotkeys [OPTIONS]... FILE_PATH
Run a simple hotkey script

Options:
    -d, --debug         Turn on debug mode
    -h, --help          Display this message"#
        );

        std::process::exit(0);
    }

    fn execute_actions(&self, enigo: &mut Enigo) {
        log::debug!("Trigger received!");

        for action in &self.script.actions {
            std::thread::sleep(Duration::from_millis(10));
            log::debug!("Action: {:?}", action);
            match action {
                Action::KeyEvent { key, direction } => enigo
                    .key(*key, *direction)
                    .expect("Could not send key event."),
                Action::MouseEvent { code, direction } => enigo
                    .button(button_from_u16(*code).unwrap_or(Button::Left), *direction)
                    .expect("Could not send mouse event."),
                Action::Sleep(duration) => {
                    std::thread::sleep(Duration::from_millis(*duration));
                }
            };
        }
    }

    pub fn run(&mut self) {
        let mut settings = Settings::default();
        settings.linux_delay = 10;

        let mut enigo = Enigo::new(&settings).expect("Could not initialize enigo.");
        let mut input = Libinput::new_with_udev(Interface);
        input
            .udev_assign_seat("seat0")
            .expect("Could not connect to seat");

        if self.script.is_repeating {
            let mut trigger_down: bool = false;

            loop {
                if input.dispatch().is_ok() {
                    for event in &mut input {
                        if let Some(is_pressed) = is_trigger(&self.script.trigger, &event) {
                            if trigger_down ^ is_pressed {
                                trigger_down = is_pressed;
                                break;
                            }
                        }
                    }

                    if trigger_down {
                        self.execute_actions(&mut enigo);
                    }
                }
            }
        } else {
            loop {
                if input.dispatch().is_ok() {
                    for event in &mut input {
                        if is_trigger(&self.script.trigger, &event).unwrap_or(false) {
                            self.execute_actions(&mut enigo);
                            break;
                        }
                    }
                }
            }
        }
    }
}
