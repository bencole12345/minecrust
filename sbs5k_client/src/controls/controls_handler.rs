use sbs5k_engine::events::{Event, EventSource};
use sbs5k_engine::inputs::Key;

use crate::constants;
use crate::controls::movement::{Rotatable, Translatable};
use crate::state::ClientState;

/// The main handler for processing input events
pub(crate) struct ControlsHandler {
    close_pressed: bool,
    player_translation_controller: WASDTranslationController,
    player_rotation_controller: MouseRotationController,
}

impl ControlsHandler {
    pub(crate) fn new() -> Self {
        ControlsHandler {
            close_pressed: false,
            player_translation_controller: WASDTranslationController::new(),
            player_rotation_controller: MouseRotationController::default(),
        }
    }

    /// Consume all events from an `EventSource`, updating the client state accordingly
    pub(crate) fn consume_events<T>(&mut self, source: &mut T)
    where
        T: EventSource,
    {
        for event in source.poll_events() {
            match event {
                Event::KeyPress(key) => self.on_key_press(key),
                Event::KeyRelease(key) => self.on_key_release(key),
                Event::MouseMove(dx, dy) => self.on_mouse_move(dx as f32, dy as f32),
            }
        }
    }

    pub(crate) fn close_has_been_pressed(&self) -> bool {
        self.close_pressed
    }

    pub(crate) fn move_player(&mut self, player: &mut (impl Rotatable + Translatable), dt: f64) {
        self.player_translation_controller.update(player, dt);
        self.player_rotation_controller.update(player);
    }

    fn on_key_press(&mut self, key: Key) {
        if key == Key::Escape {
            self.close_pressed = true;
        } else {
            self.player_translation_controller.on_key_press(key);
        }
    }

    fn on_key_release(&mut self, key: Key) {
        self.player_translation_controller.on_key_release(key);
    }

    fn on_mouse_move(&mut self, dx: f32, dy: f32) {
        self.player_rotation_controller.on_mouse_move(dx, dy);
    }
}

/// Encodes the current linear movement status of the controlled object
#[derive(Clone, Copy, Default)]
struct MovementState {
    moving_forwards: bool,
    moving_backwards: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
}

/// A controller to translate the player around the world with a standard WASD control scheme
struct WASDTranslationController {
    movement_state: MovementState,
}

impl WASDTranslationController {
    pub fn new() -> Self {
        WASDTranslationController {
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
            Key::R | Key::LeftShift => {
                self.movement_state.moving_up = true;
            }
            Key::F | Key::LeftCtrl => {
                self.movement_state.moving_down = true;
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
            Key::R | Key::LeftShift => {
                self.movement_state.moving_up = false;
            }
            Key::F | Key::LeftCtrl => {
                self.movement_state.moving_down = false;
            }

            _ => {}
        }
    }

    /// Apply the current movement state to a `Translatable` target
    fn update<T>(&self, target: &mut T, dt: f64)
    where
        T: Translatable,
    {
        let distance = constants::MOVE_SPEED * (dt as f32);

        if self.movement_state.moving_forwards && !self.movement_state.moving_backwards {
            target.translate_forwards(distance);
        }

        if self.movement_state.moving_left && !self.movement_state.moving_right {
            target.translate_left(distance);
        }

        if self.movement_state.moving_backwards && !self.movement_state.moving_forwards {
            target.translate_backwards(distance);
        }

        if self.movement_state.moving_right && !self.movement_state.moving_left {
            target.translate_right(distance);
        }

        if self.movement_state.moving_up && !self.movement_state.moving_down {
            target.translate_up(distance);
        }

        if self.movement_state.moving_down && !self.movement_state.moving_up {
            target.translate_down(distance);
        }
    }
}

/// A controller to rotate the player using standard mouse movement controls
#[derive(Default)]
struct MouseRotationController {
    accumulated_dx: f32,
    accumulated_dy: f32,
}

impl MouseRotationController {
    fn on_mouse_move(&mut self, dx: f32, dy: f32) {
        self.accumulated_dx += dx;
        self.accumulated_dy += dy;
    }

    /// Apply the current turn state to a `Rotatable` target
    fn update<T>(&mut self, target: &mut T)
    where
        T: Rotatable,
    {
        let yaw_adjustment = self.accumulated_dx * constants::TURN_SENSITIVITY;
        let pitch_adjustment = self.accumulated_dy * constants::TURN_SENSITIVITY;

        target.adjust_yaw(yaw_adjustment);
        target.adjust_pitch(pitch_adjustment);

        self.accumulated_dx = 0.0;
        self.accumulated_dy = 0.0;
    }
}
