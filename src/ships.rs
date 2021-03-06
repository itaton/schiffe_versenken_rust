
pub struct Ship {
    pub size: u8,
    pub x_start_location: u8,
    pub y_start_location: u8,
    pub vertical: bool,
    pub sunken_fields: u8,
}

impl Ship {
    pub fn new(new_size: u8, x_location: u8, y_location: u8, vertical: bool) -> Ship {
        Ship {
            size: new_size,
            x_start_location: x_location,
            y_start_location: y_location,
            vertical,
            sunken_fields: 0,
        }
    }
}