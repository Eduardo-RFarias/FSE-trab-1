use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ParkingLot {
    pub floors: Vec<Floor>,
}

impl ParkingLot {
    pub fn new() -> Self {
        let mut floors: Vec<Floor> = Vec::with_capacity(3);

        for _ in 0..3 {
            floors.push(Floor::new());
        }

        Self { floors }
    }

    pub fn is_full(&self) -> bool {
        self.floors.iter().all(|floor| floor.is_full())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Floor {
    pub spots: Vec<Spot>,
}

impl Floor {
    pub fn new() -> Self {
        let mut spots: Vec<Spot> = Vec::with_capacity(8);

        // Add 1 handicapped spot
        spots.push(Spot {
            spot_type: SpotType::Handicapped,
            parked_vehicle: None,
        });

        // Add 2 elderly spots
        for _ in 0..2 {
            spots.push(Spot {
                spot_type: SpotType::Elderly,
                parked_vehicle: None,
            });
        }

        // Add 5 normal spots
        for _ in 0..5 {
            spots.push(Spot {
                spot_type: SpotType::Normal,
                parked_vehicle: None,
            });
        }

        Self { spots }
    }

    pub fn is_full(&self) -> bool {
        self.spots.iter().all(|spot| spot.parked_vehicle.is_some())
    }

    pub fn as_bool_vec(&self) -> Vec<bool> {
        self.spots
            .iter()
            .map(|spot| spot.parked_vehicle.is_some())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spot {
    pub spot_type: SpotType,
    pub parked_vehicle: Option<Vehicle>,
}

impl Spot {
    pub fn park(&mut self, vehicle: Vehicle) {
        self.parked_vehicle = Some(vehicle);
    }

    pub fn unpark(&mut self) -> Option<Vehicle> {
        self.parked_vehicle.take()
    }
}

#[derive(Serialize, Deserialize)]
pub enum SpotType {
    Normal,
    Handicapped,
    Elderly,
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub id: i32,
    pub entry_time: i64,
}

impl Vehicle {
    pub fn calculate_fee(&self, exit_time: i64) -> f64 {
        let duration_in_minutes = (exit_time - self.entry_time) / 60;
        duration_in_minutes as f64 * 0.1
    }
}
