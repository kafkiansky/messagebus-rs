#[derive(Debug)]
pub struct Destination<'a> {
    pub exchange: &'a str,
    pub routing_key: &'a str,
}

impl<'a> Destination<'a> {
    pub fn new(exchange: &'a str, routing_key: &'a str) -> Self {
        Self {
            exchange,
            routing_key,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_destination() {
        let destination = Destination::new("events", "click");
        assert_eq!("events", destination.exchange);
        assert_eq!("click", destination.routing_key);
    }
}
