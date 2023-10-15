use std::cell::RefCell;
use std::rc::Rc;

use sbs5k_core::geometry;
use sbs5k_engine::events as engine_events;
use sbs5k_engine::inputs::Key;

use crate::constants;
use crate::controls::movement::{Rotatable, Translatable};
use crate::event;
use crate::event::Event;

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

/// The main handler for processing input events
pub(crate) struct ControlsHandler {
    movement_state: MovementState,
    mouse_accumulated_dx: f32,
    mouse_accumulated_dy: f32,
    event_submitter: event::EventSubmitter,
}

impl ControlsHandler {
    pub(crate) fn new(event_submitter: event::EventSubmitter) -> Self {
        ControlsHandler {
            movement_state: MovementState::default(),
            mouse_accumulated_dx: 0.0,
            mouse_accumulated_dy: 0.0,
            event_submitter,
        }
    }

    pub(crate) fn pump_window_events(
        &mut self,
        source: &mut impl engine_events::EventSource,
        dt: f32,
    ) {
        for event in source.poll_events() {
            match event {
                engine_events::WindowEvent::KeyPress(key) => self.on_key_press(key),
                engine_events::WindowEvent::KeyRelease(key) => self.on_key_release(key),
                engine_events::WindowEvent::MouseMove(dx, dy) => self.on_mouse_move(dx, dy),
            }
        }

        self.emit_motion_event(dt);
        self.emit_rotation_event();
    }

    fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Escape => self.event_submitter.submit_event(Event::EndGame),
            Key::W => self.movement_state.moving_forwards = true,
            Key::A => self.movement_state.moving_left = true,
            Key::S => self.movement_state.moving_backwards = true,
            Key::D => self.movement_state.moving_right = true,
            Key::R | Key::LeftShift => self.movement_state.moving_up = true,
            Key::F | Key::LeftCtrl => self.movement_state.moving_down = true,
            _ => {}
        }
    }

    fn on_key_release(&mut self, key: Key) {
        match key {
            Key::W => self.movement_state.moving_forwards = false,
            Key::A => self.movement_state.moving_left = false,
            Key::S => self.movement_state.moving_backwards = false,
            Key::D => self.movement_state.moving_right = false,
            Key::R | Key::LeftShift => self.movement_state.moving_up = false,
            Key::F | Key::LeftCtrl => self.movement_state.moving_down = false,
            _ => {}
        }
    }

    fn on_mouse_move(&mut self, dx: f32, dy: f32) {
        self.mouse_accumulated_dx += dx;
        self.mouse_accumulated_dy += dy;
    }

    fn emit_motion_event(&self, dt: f32) {
        let distance = constants::MOVE_SPEED * dt;
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;

        if self.movement_state.moving_forwards && !self.movement_state.moving_backwards {
            dx += distance;
        }
        if self.movement_state.moving_backwards && !self.movement_state.moving_forwards {
            dx -= distance;
        }

        if self.movement_state.moving_right && !self.movement_state.moving_left {
            dy += distance;
        }
        if self.movement_state.moving_left && !self.movement_state.moving_right {
            dy -= distance;
        }

        if self.movement_state.moving_up && !self.movement_state.moving_down {
            dz += distance;
        }
        if self.movement_state.moving_down && !self.movement_state.moving_up {
            dz -= distance;
        }

        if (dx, dy, dz) != (0.0, 0.0, 0.0) {
            self.event_submitter.submit_event(Event::TranslatePlayer(
                geometry::LocationDelta::new(dx, dy, dz),
            ));
        }
    }

    fn emit_rotation_event(&mut self) {
        let delta = geometry::OrientationDelta {
            delta_pitch: self.mouse_accumulated_dy * constants::TURN_SENSITIVITY,
            delta_yaw: self.mouse_accumulated_dx * constants::TURN_SENSITIVITY,
        };

        if (delta.delta_pitch, delta.delta_yaw) != (0.0, 0.0) {
            self.event_submitter
                .submit_event(Event::RotatePlayer(delta));
        }

        self.mouse_accumulated_dx = 0.0;
        self.mouse_accumulated_dy = 0.0;
    }
}

pub(crate) struct MovementApplier<T>
where
    T: Translatable + Rotatable,
{
    target: Rc<RefCell<T>>,
}

impl<T> MovementApplier<T>
where
    T: Translatable + Rotatable,
{
    pub(crate) fn new(target: Rc<RefCell<T>>) -> Self {
        Self { target }
    }
}

impl<T> event::EventListener for MovementApplier<T>
where
    T: Translatable + Rotatable,
{
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::TranslatePlayer(v) => {
                let mut target = self.target.borrow_mut();
                target.translate_forwards(v[0]);
                target.translate_right(v[1]);
                target.translate_up(v[2]);
            }
            Event::RotatePlayer(d) => {
                let mut target = self.target.borrow_mut();
                target.adjust_yaw(d.delta_yaw);
                target.adjust_pitch(d.delta_pitch);
            }
            _ => (),
        }
    }
}
