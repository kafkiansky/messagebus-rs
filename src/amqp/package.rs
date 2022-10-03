use std::collections::HashMap;
use crate::amqp;

#[derive(Debug)]
pub struct NackPolicy<'a> {
    pub requeue: bool,
    pub reason: Option<&'a str>,
}

impl<'a> NackPolicy<'a> {
    pub fn requeue(reason: Option<&'a str>) -> Self {
        Self {
            requeue: true,
            reason,
        }
    }

    pub fn dont_requeue(reason: Option<&'a str>) -> Self {
        Self {
            requeue: false,
            reason,
        }
    }
}

type AckResult = Result<(), &'static str>;

pub trait InboundPackage {
    fn id() -> String;
    fn headers() -> HashMap<String, String>;
    fn content() -> String;

    fn ack() -> AckResult;
    fn nack(nack: NackPolicy) -> AckResult;
    fn reject(nack: NackPolicy) -> AckResult;
}

#[derive(Debug)]
pub struct PublishFlags {
    pub persist: bool,
    pub mandatory: bool,
    pub immediate: bool,
}

impl Default for PublishFlags {
    fn default() -> Self {
        Self {
            persist: false,
            mandatory: false,
            immediate: false,
        }
    }
}

#[derive(Debug)]
pub struct OutboundPackage<'a> {
    pub destination: amqp::destination::Destination<'a>,
    pub publish_flags: PublishFlags,
    pub headers: HashMap<String, String>,
    pub content: String,
}

impl<'a> OutboundPackage<'a> {
    pub fn new(content: String, destination: amqp::destination::Destination<'a>) -> Self {
        Self {
            destination,
            content,
            headers: HashMap::default(),
            publish_flags: PublishFlags::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requeue() {
        let nack = NackPolicy::requeue(None);
        assert_eq!(true, nack.requeue);
        assert_eq!(None, nack.reason);

        let nack = NackPolicy::requeue(Some("error"));
        assert_eq!(true, nack.requeue);
        assert_eq!("error", nack.reason.unwrap());
    }

    #[test]
    fn dont_requeue() {
        let nack = NackPolicy::dont_requeue(None);
        assert_eq!(false, nack.requeue);
        assert_eq!(None, nack.reason);

        let nack = NackPolicy::dont_requeue(Some("panic"));
        assert_eq!(false, nack.requeue);
        assert_eq!("panic", nack.reason.unwrap());
    }

    #[test]
    fn new_outbound_package() {
        let package = OutboundPackage::new(
            String::from("some_content"),
            amqp::destination::Destination::new("test_exchange", "test_routing_key"),
        );

        assert_eq!("some_content", package.content.as_str());
        assert_eq!(false, package.publish_flags.persist);
        assert_eq!(false, package.publish_flags.immediate);
        assert_eq!(false, package.publish_flags.mandatory);
        assert_eq!("test_exchange", package.destination.exchange);
        assert_eq!("test_routing_key", package.destination.routing_key);
    }
}
