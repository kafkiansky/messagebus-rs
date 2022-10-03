use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ExchangeType {
    Direct,
    Topic,
    Fanout,
    Headers(HashMap<String, String>),
    Delayed,
}

impl fmt::Display for ExchangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct => write!(f, "direct"),
            Self::Topic => write!(f, "topic"),
            Self::Fanout => write!(f, "fanout"),
            Self::Headers(_) => write!(f, "headers"),
            Self::Delayed => write!(f, "x-delayed-message"),
        }
    }
}

#[derive(Debug)]
pub struct Exchange<'a> {
    pub name: &'a str,
    pub exchange_type: ExchangeType,
    pub passive: bool,
    pub durable: bool,
    pub auto_delete: bool,
    pub internal: bool,
    pub no_wait: bool,
    pub arguments: HashMap<String, String>,
}

impl<'a> Exchange<'a> {
    pub fn new(name: &'a str, exchange_type: ExchangeType) -> Self {
        Self {
            name,
            exchange_type,
            passive: false,
            durable: true,
            auto_delete: false,
            internal: false,
            no_wait: false,
            arguments: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct Bind<'a> {
    pub exchange: &'a str,
    pub routing_key: &'a str,
    pub no_wait: bool,
    pub arguments: HashMap<String, String>,
}

impl<'a> Bind<'a> {
    pub fn new(exchange: &'a str, routing_key: &'a str) -> Self {
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
    fn new_exchange() {
        let exchange = Exchange::new("test", ExchangeType::Direct);
        assert_eq!("test", exchange.name);
        assert_eq!("direct", exchange.exchange_type.to_string().as_str());
        assert_eq!(false, exchange.passive);
        assert_eq!(true, exchange.durable);
        assert_eq!(false, exchange.auto_delete);
        assert_eq!(false, exchange.internal);
        assert_eq!(false, exchange.no_wait);
        assert_eq!(0, exchange.arguments.len());
    }

    #[test]
    fn exchange_type() {
        assert_eq!("direct", ExchangeType::Direct.to_string().as_str());
        assert_eq!("fanout", ExchangeType::Fanout.to_string().as_str());
        assert_eq!("topic", ExchangeType::Topic.to_string().as_str());
        assert_eq!("x-delayed-message", ExchangeType::Delayed.to_string().as_str());

        let mut headers: HashMap<String, String> = HashMap::default();
        headers.insert(String::from("x-key"), String::from("value"));

        let headers_exchange_type = ExchangeType::Headers(headers);
        assert_eq!("headers", headers_exchange_type.to_string().as_str());
        match headers_exchange_type {
            ExchangeType::Headers(headers) => {
                assert_eq!("value", headers.get("x-key").unwrap().as_str());
            }
            _ => {
                assert_eq!(1, 2);
            }
        };
    }

    #[test]
    fn new_bind() {
        let bind = Bind::new("test_exchange", "test_routing_key");
        assert_eq!("test_exchange", bind.exchange);
        assert_eq!("test_routing_key", bind.routing_key);
        assert_eq!(false, bind.no_wait);
        assert_eq!(0, bind.arguments.len());
    }
}
