#[derive(Clone)]
pub(crate) struct Payload {
    pub(crate) firework_type: i32,
    pub(crate) count: u32,
}

impl Payload {
    pub(crate) fn new(firework_type: i32, count: u32) -> Self {
        Self {
            firework_type,
            count,
        }
    }
}
