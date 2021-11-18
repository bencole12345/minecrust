use glfw::{Action, Key, WindowEvent};

use crate::engine::camera::Camera;
use crate::engine::movement::Moveable;

/// Encodes the current linear and angular movement status of the controlled object
#[derive(Clone, Copy)]
struct MovementState {
    moving_forwards: bool,
    moving_backwards: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
    rotating_left: bool,
    rotating_right: bool,
    rotating_up: bool,
    rotating_down: bool,
}

impl MovementState {
    fn default() -> MovementState {
        MovementState {
            moving_forwards: false,
            moving_backwards: false,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            rotating_left: false,
            rotating_right: false,
            rotating_up: false,
            rotating_down: false,
        }
    }
}

/// Wraps a camera to control its position automatically
pub struct MovementControlledCamera {
    camera: Camera,
    movement_state: MovementState,
    movement_speed: f32,
    rotation_speed: f32,
}

impl MovementControlledCamera {
    pub fn new(movement_speed: f32, rotation_speed: f32) -> Self {
        MovementControlledCamera {
            camera: Camera::create_at_origin(),
            movement_state: MovementState::default(),
            movement_speed,
            rotation_speed,
        }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    #[rustfmt::skip]
    pub fn process_input_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::Key(key, _, Action::Press, _) => {
                match key {
                    Key::W => { self.movement_state.moving_forwards = true; }
                    Key::A => { self.movement_state.moving_left = true; }
                    Key::S => { self.movement_state.moving_backwards = true; }
                    Key::D => { self.movement_state.moving_right = true; }
                    Key::R => { self.movement_state.moving_up = true; }
                    Key::F => { self.movement_state.moving_down = true;}

                    Key::Up    => { self.movement_state.rotating_up = true; }
                    Key::Down  => { self.movement_state.rotating_down = true; }
                    Key::Left  => { self.movement_state.rotating_left = true; }
                    Key::Right => { self.movement_state.rotating_right = true; }

                    _ => {}
                }
            }

            WindowEvent::Key(key, _, Action::Release, _) => {
                match key {
                    Key::W => { self.movement_state.moving_forwards = false; }
                    Key::A => { self.movement_state.moving_left = false; }
                    Key::S => { self.movement_state.moving_backwards = false; }
                    Key::D => { self.movement_state.moving_right = false; }
                    Key::R => { self.movement_state.moving_up = false; }
                    Key::F => { self.movement_state.moving_down = false; }

                    Key::Up    => { self.movement_state.rotating_up = false; }
                    Key::Down  => { self.movement_state.rotating_down = false; }
                    Key::Left  => { self.movement_state.rotating_left = false; }
                    Key::Right => { self.movement_state.rotating_right = false; }

                    _ => {}
                }
            }

            // TODO: Handle mouse movement for rotating the camera

            _ => {}
        }
    }

    /// Adjusts the camera's position given the movement since the previous frame
    pub fn tick(&mut self, dt: f64) {
        let distance = self.movement_speed * (dt as f32);
        let angle = self.rotation_speed * (dt as f32);

        if self.movement_state.moving_forwards && !self.movement_state.moving_backwards {
            self.camera.move_forwards(distance);
        }

        if self.movement_state.moving_left && !self.movement_state.moving_right {
            self.camera.move_left(distance);
        }

        if self.movement_state.moving_backwards && !self.movement_state.moving_forwards {
            self.camera.move_backwards(distance);
        }

        if self.movement_state.moving_right && !self.movement_state.moving_left {
            self.camera.move_right(distance);
        }

        if self.movement_state.moving_up && !self.movement_state.moving_down {
            self.camera.move_up(distance);
        }

        if self.movement_state.moving_down && !self.movement_state.moving_up {
            self.camera.move_down(distance);
        }

        if self.movement_state.rotating_up && !self.movement_state.rotating_down {
            self.camera.rotate_up(angle);
        }

        if self.movement_state.rotating_down && !self.movement_state.rotating_up {
            self.camera.rotate_down(angle);
        }

        if self.movement_state.rotating_left && !self.movement_state.rotating_right {
            self.camera.rotate_left(angle);
        }

        if self.movement_state.rotating_right && !self.movement_state.rotating_left {
            self.camera.rotate_right(angle);
        }
    }
}
