use je_created_example::JECreated;
use order_created_example::OrderCreated;
use pub_sub_trait::{Publisher, Subscriber};

use crate::Bookkeeping;

impl<P> Subscriber<OrderCreated> for Bookkeeping<P>
where
    P: Publisher<JECreated>,
{
    async fn receive(&self, message: &OrderCreated) {
        self.create(message.id().to_owned()).await;
    }
}
