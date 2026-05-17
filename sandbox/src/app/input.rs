use winit::{
    event::ElementState,
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug, Default)]
pub struct InputState {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub reset_requested: bool,
    pub quit_requested: bool,
}

impl InputState {
    pub fn handle_keyboard_input(&mut self, physical_key: PhysicalKey, state: ElementState) {
        let is_pressed = state == ElementState::Pressed;

        match physical_key {
            PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.move_up = is_pressed;
            }

            PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.move_down = is_pressed;
            }

            PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.move_left = is_pressed;
            }

            PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.move_right = is_pressed;
            }

            PhysicalKey::Code(KeyCode::KeyR) => {
                if is_pressed {
                    self.reset_requested = true;
                }
            }

            PhysicalKey::Code(KeyCode::KeyQ) => {
                if is_pressed {
                    self.quit_requested = true;
                }
            }

            _ => {}
        }
    }
}
