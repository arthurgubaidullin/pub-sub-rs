use pub_sub_trait::Subscriber;
use std::fmt::Display;
use worker::console_log;

#[derive(Clone)]
pub struct ConsoleNotifications;

impl<M> Subscriber<M> for ConsoleNotifications
where
    M: Send + Sync + Clone + Display,
{
    async fn receive(&self, message: &M) {
        console_log!("[notifications]: received message: {message}");
    }
}
