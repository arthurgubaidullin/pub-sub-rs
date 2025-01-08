pub struct Orders;

impl Orders {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    pub const fn create(&self, id: usize) {}
}

impl Default for Orders {
    fn default() -> Self {
        Self::new()
    }
}
