use futures::join;
use pub_sub_trait::Subscriber;
use std::{marker::PhantomData, sync::Arc};

#[derive(Clone)]
pub struct Subscribers<M, R1, R2>
where
    M: Send + Sync + Clone,
    R1: Subscriber<M>,
    R2: Subscriber<M>,
{
    r1: Arc<R1>,
    r2: Arc<R2>,
    marker: PhantomData<M>,
}

impl<M, R1, R2> Subscribers<M, R1, R2>
where
    M: Send + Sync + Clone,
    R1: Subscriber<M>,
    R2: Subscriber<M>,
{
    pub fn new(r1: &R1, r2: &R2) -> Self {
        Self {
            r1: Arc::new(r1.clone()),
            r2: Arc::new(r2.clone()),
            marker: PhantomData,
        }
    }
}

impl<M, R1, R2> Subscriber<M> for Subscribers<M, R1, R2>
where
    M: Send + Sync + Clone,
    R1: Subscriber<M>,
    R2: Subscriber<M>,
{
    async fn receive(&self, message: &M) {
        join!(self.r1.receive(message), self.r2.receive(message));
    }
}

/// The `subscribers` macro creates nested `Subscribers` structures.
///
/// It accepts a minimum of two expressions. Each expression is passed as a reference
/// to the corresponding `Subscribers` structure. If more than two elements are provided,
/// nested structures are created.
///
/// # Examples
/// ```ignore
/// let notifications = Notifications;
/// let bookkeeping = Bookkeeping;
/// let other = Other;
///
/// let result1 = subscribers![notifications, bookkeeping];
/// // Equivalent to: Subscribers::new(&notifications, &bookkeeping)
///
/// let result2 = subscribers![notifications, bookkeeping, other];
/// // Equivalent to: Subscribers::new(&notifications, &Subscribers::new(&bookkeeping, &other))
/// ```
#[macro_export]
macro_rules! subscribers {
    ( $first:expr, $second:expr ) => {
        Subscribers::new(&$first, &$second)
    };

    ( $first:expr, $second:expr, $( $rest:expr ),+ ) => {
        Subscribers::new(
            &$first,
            &subscribers!($second, $( $rest ),+)
        )
    };
}
