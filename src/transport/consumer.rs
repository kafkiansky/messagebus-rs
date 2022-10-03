use crate::amqp::package::{self, NackPolicy};

pub enum AckMode<'a> {
    Ack,
    Nack(NackPolicy<'a>),
    Retry,
}

pub trait Consumer<'a, T>
where
    T: package::InboundPackage
{
    fn get(queue: &'a str) -> Option<T>;
    fn consume(queue: &'a str, consumer: fn(T) -> AckMode<'a>);
}
