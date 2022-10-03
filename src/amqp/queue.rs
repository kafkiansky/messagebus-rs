use std::collections::HashMap;
use crate::amqp::exchange;

#[derive(Debug, Clone)]
pub struct Queue<'a> {
    pub name: &'a str,
    pub passive: bool,
    pub durable: bool,
    pub exclusive: bool,
    pub auto_delete: bool,
    pub arguments: HashMap<String, String>,
}

impl<'a> Queue<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            passive: false,
            durable: true,
            exclusive: false,
            auto_delete: false,
            arguments: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct Bind<'a> {
    pub exchange: exchange::Exchange<'a>,
    pub routing_key: &'a str,
    pub no_wait: bool,
    pub arguments: HashMap<String, String>,
}

impl<'a> Bind<'a> {
    pub fn new(exchange: exchange::Exchange<'a>, routing_key: &'a str) -> Self {
        Self {
            exchange,
            routing_key,
            no_wait: false,
            arguments: HashMap::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue() {
        let queue = Queue::new("test");
        assert_eq!("test", queue.name);
        assert_eq!(false, queue.passive);
        assert_eq!(true, queue.durable);
        assert_eq!(false, queue.exclusive);
        assert_eq!(false, queue.auto_delete);
        assert_eq!(0, queue.arguments.len());
    }

    #[test]
    fn new_bind() {
        let exchange = exchange::Exchange::new("text_exchange", exchange::ExchangeType::Direct);

        let bind = Bind::new(exchange, "test_routing_key");
        assert_eq!("text_exchange", bind.exchange.name);
        assert_eq!("test_routing_key", bind.routing_key);
        assert_eq!(false, bind.no_wait);
        assert_eq!(0, bind.arguments.len());
    }
}
