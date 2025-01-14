use std::future::Future;

pub trait Subscriber<M>
where
    Self: Send + Sync + Clone,
    M: Send + Sync + Clone,
{
    fn receive(&self, message: &M) -> impl Future<Output = ()> + Send;
}
