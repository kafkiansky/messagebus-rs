use crate::amqp::package;

type ProduceResult = Result<(), &'static str>;

pub trait Producer {
    fn produce_one(package: package::OutboundPackage) -> ProduceResult;
    fn produce_batch(packages: Vec<package::OutboundPackage>) -> ProduceResult;
}
