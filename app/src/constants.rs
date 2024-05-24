pub struct Header {
    pub key: &'static str,
    pub value: &'static str,
}

pub const SERVER_ADDRESS: &str = "http://0.0.0.0:10380";
pub static CLIENT_HEADER: Header = Header {
    key: "X-Client-Id",
    value: "app",
};

pub const CLOSE_PARKING_LOT_EVENT: &str = "close_parking_lot";
pub const CLOSE_FLOOR_EVENT: &str = "close_floor";
pub const OPEN_PARKING_LOT_EVENT: &str = "open_parking_lot";
pub const OPEN_FLOOR_EVENT: &str = "open_floor";
pub const RESET_DATABASE_EVENT: &str = "reset_database";

pub const MENU_POS: (u16, u16) = (1, 1);
pub const FEEDBACK_POS: (u16, u16) = (1, 9);
