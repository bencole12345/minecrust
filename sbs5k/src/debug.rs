use crate::args::Args;
use crate::state::ClientState;

pub(crate) fn print_debug_output(state: &ClientState, dt: f64, config: &Args) {
    if config.debug_print_fps {
        println!("FPS: {}", 1.0 / dt);
    }
    if config.debug_print_player_position {
        let position = state.player_position.location;
        let pitch = state.player_position.orientation.pitch;
        let yaw = state.player_position.orientation.yaw;
        println!(
            "POSITION: ({:.1}, {:.1}, {:.1})  PITCH: {:.2}  YAW: {:.2}",
            position.x, position.y, position.z, pitch, yaw
        );
    }
}
