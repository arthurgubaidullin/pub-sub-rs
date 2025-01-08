use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct OrderCreated {
    id: Arc<usize>,
}

impl OrderCreated {
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self { id: Arc::new(id) }
    }

    #[must_use]
    pub fn id(&self) -> &usize {
        &self.id
    }
}

impl Display for OrderCreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "order #{} created", self.id)
    }
}
