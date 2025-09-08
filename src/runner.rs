use std::time::Duration;

use crate::parser::{Action, Script, Trigger};

use enigo::{Button, Enigo, Keyboard, Mouse, Settings};
use input::{
    Event, Libinput,
    event::{
        PointerEvent,
        keyboard::{KeyState, KeyboardEventTrait},
        pointer::ButtonState,
    },
};

pub struct Runner {
    input: Libinput,
    script: Script,
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
        5 => Some(Button::Right),
        _ => None,
    }
}

impl Runner {
    pub fn new(input: Libinput, script: Script) -> Self {
        Self { input, script }
    }

    pub fn run(&mut self) {
        let settings = Settings {
            linux_delay: 10,
            x11_display: None, //Some(":0".to_string()),
            wayland_display: Some("wayland-1".to_string()),
            windows_dw_extra_info: None,
            event_source_user_data: None,
            release_keys_when_dropped: true,
            open_prompt_to_get_permissions: true,
            independent_of_keyboard_state: true,
            windows_subject_to_mouse_speed_and_acceleration_level: false,
        };

        let mut enigo = Enigo::new(&settings).expect("Could not initialize enigo.");
        let mut trigger_down: bool = false;

        if self.script.is_repeating {
            loop {
                self.input.dispatch().unwrap();

                for event in &mut self.input {
                    if let Some(is_pressed) = is_trigger(&self.script.trigger, &event) {
                        if trigger_down ^ is_pressed {
                            trigger_down = is_pressed;
                            break;
                        }
                    }
                }

                if trigger_down {
                    for action in &self.script.actions {
                        std::thread::sleep(Duration::from_millis(10));
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
            }
        } else {
            loop {
                self.input.dispatch().unwrap();
                for event in &mut self.input {
                    if is_trigger(&self.script.trigger, &event).unwrap_or(false) {
                        for action in &self.script.actions {
                            std::thread::sleep(Duration::from_millis(10));
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
                }
            }
        }
    }
}
