use std::sync::Arc;

use je_created_example::JECreated;
use pub_sub_trait::Publisher;

#[derive(Clone)]
pub struct Bookkeeping<P>
where
    P: Publisher<JECreated>,
{
    je_created_topic: Arc<P>,
}

impl<P> Bookkeeping<P>
where
    P: Publisher<JECreated>,
{
    pub fn new(publisher: P) -> Self {
        Self {
            je_created_topic: Arc::new(publisher),
        }
    }

    pub async fn create(&self, id: usize) {
        let je_created = JECreated::new(id);

        self.je_created_topic.publish(&je_created).await;
    }
}
