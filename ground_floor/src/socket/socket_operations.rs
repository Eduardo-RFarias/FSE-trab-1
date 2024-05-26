pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}

pub static SERVER_URL: &str = "http://0.0.0.0:10380";
pub static CLOSING_PARKING_LOT: &str = "close_parking_lot";
pub static OPENING_PARKING_LOT: &str = "open_parking_lot";
pub static CAR_ARRIVED: &str = "car_arrived";
pub static CAR_DEPARTED: &str = "car_departed";
pub static FLOOR_STATE: &str = "floor_state";
pub static CLIENT_HEADER: Header = Header {
    key: "X-Client-Id",
    value: "ground_floor",
};
