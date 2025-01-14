use pub_sub_trait::{Publisher, Subscriber};
use std::{
    marker::PhantomData,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct Topic<M, S>
where
    M: Send + Sync + Clone,
    S: Subscriber<M>,
{
    subscriber: Arc<RwLock<Option<S>>>,
    marker: PhantomData<M>,
}

impl<M, S> Topic<M, S>
where
    M: Send + Sync + Clone,
    S: Subscriber<M>,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscriber: Arc::new(RwLock::new(None)),
            marker: PhantomData,
        }
    }

    ///
    /// # Panics
    ///
    /// This function might panic when called if the internal lock is already held by the current thread.
    pub fn link(&mut self, receiver: &S) {
        let mut lock = self.subscriber.write().unwrap();

        *lock = Some(receiver.clone());
    }
}

impl<M, S> Default for Topic<M, S>
where
    M: Send + Sync + Clone,
    S: Subscriber<M>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<M, S> Publisher<M> for Topic<M, S>
where
    M: Send + Sync + Clone,
    S: Subscriber<M>,
{
    async fn publish(&self, message: &M) {
        let subscriber = {
            let lock = self.subscriber.read().unwrap();

            lock.clone()
        };

        if let Some(subscriber) = subscriber {
            subscriber.receive(message).await;
        }
    }
}
