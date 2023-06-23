use specs::prelude::Entity;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Packet {
    pub msg: String,
    pub size: u64,
    pub from: Entity,
    pub to: Entity,
}