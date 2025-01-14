use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use bookkeeping_example::Bookkeeping;

use crate::console_notifications::ConsoleNotifications;
use in_memory_pub_sub::Subscribers;
use in_memory_pub_sub::{subscribers, Topic};
use order_created_example::OrderCreated;
use orders_example::Orders;
use pub_sub_trait::Publisher;

pub fn router() -> Router {
    let mut order_created_topic = Topic::new();
    let mut je_created_topic = Topic::new();

    let notifications = ConsoleNotifications;
    let bookkeeping = Bookkeeping::new(je_created_topic.clone());
    let orders = Orders::new(order_created_topic.clone());

    let order_created_subscription = subscribers![notifications, bookkeeping];

    je_created_topic.link(&notifications);
    order_created_topic.link(&order_created_subscription);

    Router::new()
        .route("/orders", get(orders_handler))
        .with_state(orders)
}

async fn orders_handler<P>(State(orders): State<Orders<P>>) -> impl IntoResponse
where
    P: Publisher<OrderCreated>,
{
    orders.create(1).await;

    (StatusCode::CREATED, "Order created.")
}
