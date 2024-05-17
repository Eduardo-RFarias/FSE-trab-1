use crate::models::{client::ClientId, parking_lot::ParkingLot};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI32, Arc, Mutex},
};

pub struct Database {
    pub parking_lot: ParkingLot,
    pub clients: HashMap<String, ClientId>,
    pub id_counter: Arc<AtomicI32>,
}

impl Database {
    pub fn new() -> Arc<Mutex<Self>> {
        let instance = Self {
            parking_lot: ParkingLot::new(),
            clients: HashMap::with_capacity(3),
            id_counter: Arc::new(AtomicI32::new(1)),
        };

        Arc::new(Mutex::new(instance))
    }
}
