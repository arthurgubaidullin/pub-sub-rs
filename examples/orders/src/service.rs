use order_created_example::OrderCreated;
use pub_sub_trait::Publisher;
use std::sync::Arc;

#[derive(Clone)]
pub struct Orders<P>
where
    Self: Send + Sync,
    P: Publisher<OrderCreated>,
{
    order_created_topic: Arc<P>,
}

impl<P> Orders<P>
where
    Self: Send + Sync,
    P: Publisher<OrderCreated>,
{
    #[must_use]
    pub fn new(publisher: P) -> Self {
        Self {
            order_created_topic: Arc::new(publisher),
        }
    }

    pub async fn create(&self, id: usize) {
        let message = OrderCreated::new(id);

        self.order_created_topic.publish(&message).await;
    }
}
