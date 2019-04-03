
struct Ship {
    size: i8;
    x_start_location: i8;
    y_start_location: i8;
}

impl Ship {
    fn new(new_size: i8, x_location: i8, y_location: i8) -> Ship {
        Ship {
            size: new_size,
            x_start_location = x_location,
            y_start_location = y_location,
        }
    }
}