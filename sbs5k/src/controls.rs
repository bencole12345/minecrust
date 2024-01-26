use glm::fmod;
use sbs5k_core::geometry;
use sbs5k_engine::events as engine_events;
use sbs5k_engine::inputs::Key;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use crate::constants;
use crate::event;
use crate::event::Event;
use crate::updatable::Updatable;

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
    controlled_target: Rc<RefCell<geometry::EntityPosition>>,
    movement_state: MovementState,
    mouse_accumulated_dx: f32,
    mouse_accumulated_dy: f32,
    player_position: geometry::EntityPosition,
    event_submitter: event::EventSubmitter,
}

impl ControlsHandler {
    pub(crate) fn new(
        controlled_target: Rc<RefCell<geometry::EntityPosition>>,
        event_submitter: event::EventSubmitter,
    ) -> Self {
        let current_pos = *controlled_target.borrow();
        ControlsHandler {
            controlled_target,
            movement_state: MovementState::default(),
            mouse_accumulated_dx: 0.0,
            mouse_accumulated_dy: 0.0,
            player_position: current_pos,
            event_submitter,
        }
    }

    pub(crate) fn pump_window_events(&mut self, source: &mut impl engine_events::EventSource) {
        for event in source.poll_events() {
            match event {
                engine_events::WindowEvent::KeyPress(key) => self.on_key_press(key),
                engine_events::WindowEvent::KeyRelease(key) => self.on_key_release(key),
                engine_events::WindowEvent::MouseMove(dx, dy) => self.on_mouse_move(dx, dy),
            }
        }
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
}

impl Updatable for ControlsHandler {
    fn update(&mut self, dt: f32) {
        let original_position = self.player_position;

        // Translation
        let distance = constants::MOVE_SPEED * dt;
        match (
            self.movement_state.moving_right,
            self.movement_state.moving_left,
        ) {
            (true, false) => self.player_position.translate_right(distance),
            (false, true) => self.player_position.translate_right(-distance),
            _ => {}
        }
        match (
            self.movement_state.moving_forwards,
            self.movement_state.moving_backwards,
        ) {
            (true, false) => self.player_position.translate_forwards(distance),
            (false, true) => self.player_position.translate_forwards(-distance),
            _ => {}
        };
        match (
            self.movement_state.moving_up,
            self.movement_state.moving_down,
        ) {
            (true, false) => self.player_position.translate_up(distance),
            (false, true) => self.player_position.translate_up(-distance),
            _ => {}
        };

        // Rotation (consume all the mouse movement events that have accumulated this tick)
        if self.mouse_accumulated_dx != 0.0 {
            let angle = self.mouse_accumulated_dx * constants::TURN_SENSITIVITY;
            self.player_position.orientation.yaw =
                fmod(self.player_position.orientation.yaw + angle, 2.0 * PI);
        }
        if self.mouse_accumulated_dy != 0.0 {
            let angle = self.mouse_accumulated_dy * constants::TURN_SENSITIVITY;
            self.player_position.orientation.pitch = glm::clamp(
                self.player_position.orientation.pitch + angle,
                -PI * 0.49,
                PI * 0.49,
            );
        }
        (self.mouse_accumulated_dx, self.mouse_accumulated_dy) = (0.0, 0.0);

        if self.player_position != original_position {
            *self.controlled_target.borrow_mut() = self.player_position;
            self.event_submitter
                .submit_event(Event::PlayerChangedPosition(self.player_position));
        }
    }
}
