
pub struct Ship {
    size: u8,
    x_start_location: u8,
    y_start_location: u8,
    direction: Direction,
}
//enum for direction ?
enum Direction {
    vertical,
    horizontal
}

pub impl Ship {
    fn new(new_size: u8, x_location: u8, y_location: u8) -> Ship {
        Ship {
            size: new_size,
            x_start_location: x_location,
            y_start_location: y_location,
        }
    }
}