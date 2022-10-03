use std::collections::HashMap;

use crate::amqp;

type OperatorResult = Result<(), &'static str>;

#[derive(Debug)]
pub struct BindArgs<'a> {
    pub routing_key: &'a str,
    pub no_wait: bool,
    pub arguments: HashMap<String, String>,
}

impl<'a> BindArgs<'a> {
    pub fn new(routing_key: &'a str) -> Self {
        Self {
            routing_key,
            no_wait: false,
            arguments: HashMap::default(),
        }
    }
}

pub trait AmqpOperator {
    fn declare_queue(&mut self, queue: &amqp::queue::Queue) -> OperatorResult;
    fn declare_exchange(&mut self, exchange: &amqp::exchange::Exchange) -> OperatorResult;
    fn bind_queue(&mut self, queue: &amqp::queue::Queue, exchange: &amqp::exchange::Exchange, bind_args: BindArgs) -> OperatorResult;
    fn bind_exchange(&mut self, source: &amqp::exchange::Exchange, destination: &amqp::exchange::Exchange, bind_args: BindArgs) -> OperatorResult;
}
