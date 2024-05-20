use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Floor {
    pub floor_number: i32,
    pub spots: Vec<Spot>,
}

impl Floor {
    pub fn as_bool_vec(&self) -> Vec<bool> {
        self.spots
            .iter()
            .map(|spot| spot.parked_vehicle.is_some())
            .collect()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spot {
    pub spot_number: i32,
    pub spot_type: SpotType,
    pub parked_vehicle: Option<Vehicle>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum SpotType {
    Normal = 0,
    Handicapped = 1,
    Elderly = 2,
}

impl FromSql for SpotType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let number = value.as_i64()?;

        match number {
            0 => Ok(SpotType::Normal),
            1 => Ok(SpotType::Handicapped),
            2 => Ok(SpotType::Elderly),
            _ => Err(FromSqlError::OutOfRange(number)),
        }
    }
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
