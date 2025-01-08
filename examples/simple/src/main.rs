use orders_example::Orders;

#[tokio::main]
async fn main() {
    let orders = Orders;

    orders.create(1);
}
