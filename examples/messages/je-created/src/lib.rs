use std::{fmt::Display, sync::Arc};

#[derive(Debug, Clone)]
pub struct JECreated {
    id: Arc<usize>,
}

impl JECreated {
    #[must_use]
    pub fn new(id: usize) -> Self {
        Self { id: Arc::new(id) }
    }

    #[must_use]
    pub fn id(&self) -> &usize {
        &self.id
    }
}

impl Display for JECreated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "journal entry #{} created", self.id)
    }
}
