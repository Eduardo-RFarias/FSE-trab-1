use chrono::Utc;
use global_counter::primitive::exact::CounterI32;
use serde::{Deserialize, Serialize};

static CAR_ID: CounterI32 = CounterI32::new(1);

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub id: i32,
    pub entry_time: i64,
}

impl Car {
    pub fn new() -> Car {
        Car {
            id: CAR_ID.inc(),
            entry_time: Utc::now().timestamp(),
        }
    }
}

impl Clone for Car {
    fn clone(&self) -> Car {
        Car {
            id: self.id,
            entry_time: self.entry_time,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ParkingSpaceType {
    Normal,
    Elderly,
    Handicapped,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParkingSpace {
    pub space_type: ParkingSpaceType,
    pub car: Option<Car>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParkingLot {
    pub spaces: Vec<ParkingSpace>,
}

impl ParkingLot {
    pub fn new() -> ParkingLot {
        let mut spaces = Vec::with_capacity(8);

        spaces.push(ParkingSpace {
            space_type: ParkingSpaceType::Handicapped,
            car: None,
        });

        for _ in 0..2 {
            spaces.push(ParkingSpace {
                space_type: ParkingSpaceType::Elderly,
                car: None,
            });
        }

        for _ in 0..5 {
            spaces.push(ParkingSpace {
                space_type: ParkingSpaceType::Normal,
                car: None,
            });
        }

        ParkingLot { spaces }
    }
}
