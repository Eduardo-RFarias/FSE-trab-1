use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};

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

pub struct Spot {
    pub spot_number: i32,
    pub spot_type: SpotType,
    pub parked_vehicle: Option<Vehicle>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SpotType {
    Normal = 0,
    Handicapped = 1,
    Elderly = 2,
}

impl From<SpotType> for i32 {
    fn from(spot_type: SpotType) -> i32 {
        match spot_type {
            SpotType::Normal => 0,
            SpotType::Handicapped => 1,
            SpotType::Elderly => 2,
        }
    }
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

pub struct Vehicle {
    pub id: i32,
    pub entry_time: i64,
}
