use bookkeeping_example::Bookkeeping;
use futures::join;
use in_memory_pub_sub::{subscribers, Subscribers, Topic};
use notifications_example::Notifications;
use orders_example::Orders;
use std::time::Duration;
use tokio::{main, time::sleep};

#[main]
async fn main() {
    let mut order_created_topic = Topic::new();
    let mut je_created_topic = Topic::new();

    let notifications = Notifications;
    let bookkeeping = Bookkeeping::new(je_created_topic.clone());
    let orders = Orders::new(order_created_topic.clone());

    let order_created_subscription = subscribers![notifications, bookkeeping];

    je_created_topic.link(&notifications);
    order_created_topic.link(&order_created_subscription);

    join!(
        async {
            sleep(Duration::from_millis(1)).await;
            orders.create(1).await;
        },
        orders.create(2),
        orders.create(3)
    );
}
