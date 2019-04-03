#![warn(clippy::all)]
struct ShootPacket {
    line: u8;
    column: u8;     
}

struct FeedbackPacket {
    hit: bool;
}

struct WhoAmIPacket {
    is_server: bool;
}

impl ShootPacket {
    fn new(line: u8, column: u8) -> ShootPacket {
        ShootPacket {
            line: line,
            line: column
        }
    }
}

impl FeedbackPacket {
    fn new(hit: bool) -> FeedbackPacket {
        FeedbackPacket {
            hit: hit
        }
    }
}

impl WhoAmIPacket {
    fn new(is_server: bool) -> WhoAmIPacket {
        WhoAmIPacket {
            is_server: is_server;
        }
    }
}