/// Parameters about fog
pub struct FogParameters {
    /// The distance at which objects start to have fog applied on top
    pub start_threshold: f32,

    /// The distance above which there will be total fog and the object will not be visible
    pub end_threshold: f32,
}
