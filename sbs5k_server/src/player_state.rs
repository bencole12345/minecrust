use sbs5k_core::geometry;

#[derive(Clone, Default)]
pub struct PlayerState {
    pub username: String,
    pub current_position: geometry::EntityPosition,
}
