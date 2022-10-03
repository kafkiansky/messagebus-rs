use crate::routing::operator;
use crate::amqp::{queue, exchange};

type ConfiguratorResult = Result<(), &'static str>;

pub struct AmqpConfigurator<T>
where
    T: operator::AmqpOperator,
{
    operator: T,
}

impl<T> AmqpConfigurator<T>
where
    T: operator::AmqpOperator
{
    pub fn new(operator: T) -> Self {
        Self {
            operator,
        }
    }

    pub fn bind_queue(&mut self, queue: queue::Queue, bindings: Vec<queue::Bind>) -> ConfiguratorResult {
        if let Err(e) = self.operator.declare_queue(&queue) {
            return Err(e);
        }

        for binding in bindings {
            if let Err(e) = self.operator.declare_exchange(&binding.exchange) {
                return Err(e);
            }

            if let Err(e) = self.operator.bind_queue(&queue, &binding.exchange, operator::BindArgs {
                routing_key: binding.routing_key,
                no_wait: binding.no_wait,
                arguments: binding.arguments,
            }) {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn bind_exchange(&mut self, exchange: exchange::Exchange, bindings: Vec<exchange::Bind>) -> ConfiguratorResult {
        if let Err(e) = self.operator.declare_exchange(&exchange) {
            return Err(e);
        }

        for binding in bindings {
            if let Err(e) = self.operator.declare_exchange(&binding.exchange) {
                return Err(e);
            }

            if let Err(e) = self.operator.bind_exchange(&exchange, &binding.exchange, operator::BindArgs {
                routing_key: binding.routing_key,
                no_wait: binding.no_wait,
                arguments: binding.arguments,
            }) {
                return Err(e);
            }
        }

        Ok(())
    }
}
