// mod coordinator;

use std::marker::PhantomData;

use super::coordinator::Coordinator;

#[derive(Debug)]
pub struct Client {
    name: String,
    // phantom: PhantomData<T>,
}

impl Client{
    pub fn commit(coordinator: &mut Coordinator, msg: &str) {
        coordinator.commit(msg);
    }
}