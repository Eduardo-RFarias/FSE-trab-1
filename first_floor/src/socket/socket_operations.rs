pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}

pub static SERVER_URL: &str = "http://0.0.0.0:10380";
pub static CLOSING_FLOOR: &str = "close_floor";
pub static OPENING_FLOOR: &str = "open_floor";
pub static CAR_ARRIVED: &str = "car_arrived";
pub static CAR_DEPARTED: &str = "car_departed";
pub static FLOOR_STATE: &str = "floor_state";
pub static CLIENT_HEADER: Header = Header {
    key: "X-Client-Id",
    value: "first_floor",
};
