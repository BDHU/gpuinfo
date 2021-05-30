// This struct is created for every single GPU in a system
pub struct GPUstat {
    name: String,
    id: u32,
    fan_speed: u32,
    power_limit: u32,
    memory_info: String,
    // TODO: add more fields
}
