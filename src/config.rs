#[derive(Clone, Debug)]
pub struct ConfigParameters {
    pub bot_maintainers: Vec<u64>,
}

impl ConfigParameters {
    pub fn new(bot_maintainers: Vec<u64>) -> Self {
        Self { bot_maintainers }
    }
}
