use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParkingSpaceModifiedPayload {
    pub parking_space: i32,
    pub timestamp: i64,
}
