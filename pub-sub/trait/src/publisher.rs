use std::future::Future;

pub trait Publisher<M>
where
    Self: Send + Sync + Clone,
    M: Send + Sync + Clone,
{
    fn publish(&self, message: &M) -> impl Future<Output = ()> + Send;
}
