use crate::Notifications;
use pub_sub_trait::Subscriber;
use std::fmt::Display;

impl<M> Subscriber<M> for Notifications
where
    M: Send + Sync + Clone + Display,
{
    async fn receive(&self, message: &M) {
        println!("[notifications]: received message: {message}");
    }
}
