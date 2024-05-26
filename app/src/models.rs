use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParkingLotDataPayload {
    pub floors: Vec<FloorDataPayload>,
    pub exited_vehicles: Vec<VehicleDataPayload>,
    pub is_closed: bool,
}

impl ParkingLotDataPayload {
    pub fn new() -> Self {
        Self {
            floors: vec![
                FloorDataPayload::new(),
                FloorDataPayload::new(),
                FloorDataPayload::new(),
            ],
            exited_vehicles: vec![],
            is_closed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FloorDataPayload {
    pub spots: Vec<SpotDataPayload>,
    pub is_closed: bool,
}

impl FloorDataPayload {
    pub fn new() -> Self {
        let mut instance = Self {
            spots: vec![],
            is_closed: false,
        };

        for _ in 0..8 {
            instance.spots.push(SpotDataPayload::new(0));
        }

        instance
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpotDataPayload {
    pub spot_type: i32,
    pub parked_vehicle: Option<VehicleDataPayload>,
}

impl SpotDataPayload {
    pub fn new(spot_type: i32) -> Self {
        Self {
            spot_type,
            parked_vehicle: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VehicleDataPayload {
    pub id: i32,
    pub entry_time: i64,
    pub exit_time: Option<i64>,
}

impl VehicleDataPayload {
    pub fn fee(&self) -> f64 {
        let entry_time = self.entry_time;
        let exit_time = self.exit_time.unwrap_or_else(|| Utc::now().timestamp());

        ((exit_time - entry_time) / 60) as f64 * 0.1
    }
}
