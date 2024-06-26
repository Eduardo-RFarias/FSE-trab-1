use rust_socketio::Payload;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
pub struct ParkingLot {
    pub spaces: Vec<bool>,
}

impl ParkingLot {
    pub fn new() -> Arc<Mutex<ParkingLot>> {
        let mut spaces = Vec::with_capacity(8);

        for _ in 0..8 {
            spaces.push(false);
        }

        let parking_lot = ParkingLot { spaces };

        Arc::new(Mutex::new(parking_lot))
    }

    pub fn update_spaces(&mut self, spaces: &Vec<bool>) {
        for i in 0..8 {
            self.spaces[i] = spaces[i];
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ParkingSpaceModifiedPayload {
    pub parking_space: i32,
    pub timestamp: i64,
}

impl Into<Payload> for ParkingSpaceModifiedPayload {
    fn into(self) -> Payload {
        Payload::from(serde_json::to_value(self).unwrap())
    }
}
