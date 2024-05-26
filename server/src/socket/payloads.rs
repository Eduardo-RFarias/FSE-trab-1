use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParkingSpaceModifiedPayload {
    pub parking_space: i32,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ParkingLotDataPayload {
    pub floors: Vec<FloorDataPayload>,
    pub exited_vehicles: Vec<VehicleDataPayload>,
    pub is_closed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FloorDataPayload {
    pub spots: Vec<SpotDataPayload>,
    pub is_closed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SpotDataPayload {
    pub spot_type: i32,
    pub parked_vehicle: Option<VehicleDataPayload>,
}

#[derive(Serialize, Deserialize)]
pub struct VehicleDataPayload {
    pub id: i32,
    pub entry_time: i64,
    pub exit_time: Option<i64>,
}
