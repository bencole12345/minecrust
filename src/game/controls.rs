// use crate::engine::events::{Key, KeyPressHandler};
use crate::engine::camera::Camera;
use crate::engine::events::{Event, EventSource};
use crate::engine::inputs::Key;
use crate::engine::movement::Moveable;

#[derive(Clone, Copy, Debug)]
pub(crate) struct MovementSpeed {
    linear_speed: f32,
    angular_speed: f32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        MovementSpeed {
            linear_speed: 1.0,
            angular_speed: 1.0,
        }
    }
}

/// Implements a standard control scheme for moving the camera around the world.
pub(crate) struct Controls {
    close_pressed: bool,

    camera_movement_controller: WASDControlledMovementState,
    camera_movement_speed: MovementSpeed,
}

impl Controls {
    pub(crate) fn new() -> Self {
        Controls {
            close_pressed: false,
            camera_movement_controller: WASDControlledMovementState::new(),
            camera_movement_speed: MovementSpeed::default(),
        }
    }

    pub(crate) fn consume_events<T>(&mut self, source: &mut T)
    where
        T: EventSource,
    {
        for event in source.poll_events() {
            match event {
                Event::KeyPress(key) => self.on_key_press(key),
                Event::KeyRelease(key) => self.on_key_release(key),
            }
        }
    }

    pub(crate) fn close_has_been_pressed(&self) -> bool {
        self.close_pressed
    }

    fn on_key_press(&mut self, key: Key) {
        if key == Key::Escape {
            self.close_pressed = true;
        } else {
            self.camera_movement_controller.on_key_press(key);
        }
    }

    fn on_key_release(&mut self, key: Key) {
        self.camera_movement_controller.on_key_release(key);
    }

    pub fn update_camera(&self, camera: &mut Camera, dt: f64) {
        self.camera_movement_controller.movement_state.apply(
            camera,
            dt,
            self.camera_movement_speed,
        );
    }
}

/// Encodes the current linear and angular movement status of the controlled object
#[derive(Clone, Copy, Default)]
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
    fn apply<T>(&self, moveable: &mut T, dt: f64, movement_speed: MovementSpeed)
    where
        T: Moveable,
    {
        let distance = movement_speed.linear_speed * (dt as f32);
        let angle = movement_speed.angular_speed * (dt as f32);

        if self.moving_forwards && !self.moving_backwards {
            moveable.move_forwards(distance);
        }

        if self.moving_left && !self.moving_right {
            moveable.move_left(distance);
        }

        if self.moving_backwards && !self.moving_forwards {
            moveable.move_backwards(distance);
        }

        if self.moving_right && !self.moving_left {
            moveable.move_right(distance);
        }

        if self.moving_up && !self.moving_down {
            moveable.move_up(distance);
        }

        if self.moving_down && !self.moving_up {
            moveable.move_down(distance);
        }

        if self.rotating_up && !self.rotating_down {
            moveable.rotate_up(angle);
        }

        if self.rotating_down && !self.rotating_up {
            moveable.rotate_down(angle);
        }

        if self.rotating_left && !self.rotating_right {
            moveable.rotate_left(angle);
        }

        if self.rotating_right && !self.rotating_left {
            moveable.rotate_right(angle);
        }
    }
}

struct WASDControlledMovementState {
    movement_state: MovementState,
}

impl WASDControlledMovementState {
    pub fn new() -> Self {
        WASDControlledMovementState {
            movement_state: MovementState::default(),
        }
    }

    #[inline]
    pub fn on_key_press(&mut self, key: Key) {
        match key {
            Key::W => {
                self.movement_state.moving_forwards = true;
            }
            Key::A => {
                self.movement_state.moving_left = true;
            }
            Key::S => {
                self.movement_state.moving_backwards = true;
            }
            Key::D => {
                self.movement_state.moving_right = true;
            }
            Key::R => {
                self.movement_state.moving_up = true;
            }
            Key::F => {
                self.movement_state.moving_down = true;
            }
            Key::Up => {
                self.movement_state.rotating_up = true;
            }
            Key::Down => {
                self.movement_state.rotating_down = true;
            }
            Key::Left => {
                self.movement_state.rotating_left = true;
            }
            Key::Right => {
                self.movement_state.rotating_right = true;
            }

            _ => {}
        }
    }

    #[inline]
    pub fn on_key_release(&mut self, key: Key) {
        match key {
            Key::W => {
                self.movement_state.moving_forwards = false;
            }
            Key::A => {
                self.movement_state.moving_left = false;
            }
            Key::S => {
                self.movement_state.moving_backwards = false;
            }
            Key::D => {
                self.movement_state.moving_right = false;
            }
            Key::R => {
                self.movement_state.moving_up = false;
            }
            Key::F => {
                self.movement_state.moving_down = false;
            }
            Key::Up => {
                self.movement_state.rotating_up = false;
            }
            Key::Down => {
                self.movement_state.rotating_down = false;
            }
            Key::Left => {
                self.movement_state.rotating_left = false;
            }
            Key::Right => {
                self.movement_state.rotating_right = false;
            }

            _ => {}
        }
    }
}
