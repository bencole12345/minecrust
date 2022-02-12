use super::state::ClientState;

// TODO: Make these command line args
const PRINT_FPS: bool = false;
const PRINT_POS: bool = false;

pub const DEBUGGING_ENABLED: bool = PRINT_FPS || PRINT_POS;

pub(crate) fn print_debug_output(state: &ClientState, dt: f64) {
    if PRINT_FPS {
        println!("FPS: {}", 1.0 / dt);
    }
    if PRINT_POS {
        let position = state.player_position.position;
        let pitch = state.player_position.pitch;
        let yaw = state.player_position.yaw;
        println!(
            "POSITION: ({:.1}, {:.1}, {:.1})  PITCH: {:.2}  YAW: {:.2}",
            position.x, position.y, position.z, pitch, yaw
        );
    }
}
