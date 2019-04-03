#![warn(clippy::all)]

pub struct ShootPacket {
    line: u8,
    column: u8,     
}

pub struct FeedbackPacket {
    hit: bool,
}

pub struct WhoAmIPacket {
    is_server: bool,
}

impl ShootPacket {
    fn new(l: u8, c: u8) -> ShootPacket {
        ShootPacket {
            line: l,
            column: c,
        }
    }
}

impl FeedbackPacket {
    fn new(h: bool) -> FeedbackPacket {
        FeedbackPacket {
            hit: h
        }
    }
}

impl WhoAmIPacket {
    fn new(serv: bool) -> WhoAmIPacket {
        WhoAmIPacket {
            is_server: serv
        }
    }
}