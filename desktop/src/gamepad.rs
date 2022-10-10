use std::sync::{Arc, Mutex};
use gilrs::{Axis, Button, Gilrs};
use gilrs::EventType::{AxisChanged, ButtonPressed, ButtonReleased};
use ruffle_core::Player;
use ruffle_core::external::{Value as ExternalValue};

pub struct Gamepad {
    gilrs: Mutex<Gilrs>,
}

impl Gamepad {

    pub fn new() -> Self {
        Self {
            gilrs: Mutex::new(Gilrs::new().unwrap()),
        }
    }

    pub fn axis_to_str(&self, axis: Axis) -> String {
        match axis {
            Axis::LeftStickX => "lx",
            Axis::LeftStickY => "ly",
            Axis::LeftZ => "lz",
            Axis::RightStickX => "rx",
            Axis::RightStickY => "ry",
            Axis::RightZ => "rz",
            _ => "N/A"
        }.to_string()
    }

    pub fn button_to_str(&self, button: Button) -> String {
        match button {
            Button::DPadUp => "up",
            Button::DPadLeft => "left",
            Button::DPadRight => "right",
            Button::DPadDown => "down",
            Button::LeftTrigger => "l1",
            Button::LeftTrigger2 => "l2",
            Button::LeftThumb => "l3",
            Button::RightTrigger => "r1",
            Button::RightTrigger2 => "r2",
            Button::RightThumb => "r3",
            Button::Select => "select",
            Button::Start => "start",
            Button::North => "y",
            Button::East => "b",
            Button::South => "a",
            Button::West => "x",
            _ => "N/A"
        }.to_string()
    }

    pub fn handle_gamepad_input(&mut self, player: &Arc<Mutex<Player>>) {

        let mut player_lock = player.lock().unwrap();
        let mut gilrs_lock = self.gilrs.lock().unwrap();

        while let Some(ev) = gilrs_lock.next_event() {

            let gamepad_id: usize = usize::from(ev.id);

            match ev.event {
                ButtonPressed(button, _code) => {

                    player_lock.call_internal_interface("gamepad.onChange", [
                        ExternalValue::from(gamepad_id),
                        ExternalValue::String(self.button_to_str(button)),
                        ExternalValue::Bool(true),
                    ]);

                },
                ButtonReleased(button, _code) => {

                    player_lock.call_internal_interface("gamepad.onChange", [
                        ExternalValue::from(gamepad_id),
                        ExternalValue::String(self.button_to_str(button)),
                        ExternalValue::Bool(false),
                    ]);

                },
                AxisChanged(axis, value, _code) => {

                    player_lock.call_internal_interface("gamepad.onChange", [
                        ExternalValue::from(gamepad_id),
                        ExternalValue::String(self.axis_to_str(axis)),
                        ExternalValue::from(value),
                    ]);

                },
                _ => ()
            }

        }

    }

}
