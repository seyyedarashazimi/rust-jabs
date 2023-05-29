use std::fmt::Debug;

pub trait Event: Debug + Clone {
    fn execute(&self) {}
}
