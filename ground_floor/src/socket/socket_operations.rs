pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}

pub static SERVER_URL: &str = "http://localhost:10380";
pub static CLOSING_PARKING_LOT: &str = "close_parking_lot";
pub static OPENING_PARKING_LOT: &str = "open_parking_lot";
pub static CAR_ARRIVED: &str = "car_arrived";
pub static CAR_DEPARTED: &str = "car_departed";
pub static PARKING_LOT_STATE: &str = "parking_lot_state";
pub static CLIENT_HEADER: Header = Header {
    key: "X-Client-Id",
    value: "ground_floor",
};
