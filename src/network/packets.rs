#![warn(clippy::all)]
#![allow(dead_code)]

use alloc::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub struct ShootPacket {
    line: u8,
    column: u8,     
}

#[derive(Debug, Copy, Clone)]
pub struct FeedbackPacket {
    hit: bool,
    sunk: u8,
    you_win: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct WhoamiPacket {
    pub is_server: bool,
}

impl ShootPacket {
    pub fn new(l: u8, c: u8) -> ShootPacket {
        ShootPacket {
            line: l,
            column: c,
        }
    }
}

impl FeedbackPacket {
    pub fn new(h: bool, s: u8, w: bool) -> FeedbackPacket {
        FeedbackPacket {
            hit: h,
            sunk: s,
            you_win: w,
        }
    }
}

impl WhoamiPacket {
    fn new(serv: bool) -> WhoamiPacket {
        WhoamiPacket {
            is_server: serv
        }
    }
}

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(input: &[u8]) -> Self;
    fn len() -> usize;
}

impl Serializable for ShootPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.line);
        result.push(self.column);
        result
    }

    fn deserialize(input: &[u8]) -> Self {
        ShootPacket {
            line: input[0],
            column: input[1],
        }
    }

    fn len() -> usize {
        2
    }
}

impl Serializable for FeedbackPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        if self.hit {
            result.push(255);
        }
        else {
            result.push(0)
        }
        result.push(self.sunk);
        if self.you_win {
            result.push(255);
        }
        else {
            result.push(0);
        }
        result
    }

    fn deserialize(input: &[u8]) -> Self {
        FeedbackPacket {
            hit: input[0] == 255,
            sunk: input[1],
            you_win: input[2] == 255,
        }
    }

    fn len() -> usize {
        3
    }
}

impl Serializable for WhoamiPacket {
    fn serialize(&self) -> Vec<u8> {
        if self.is_server {
            vec![255]
        } else {
            vec![0]
        }
    }

    fn deserialize(input: &[u8]) -> WhoamiPacket {
        WhoamiPacket {
            is_server: input[0] == 255,
        }
    }

    fn len() -> usize {
        1
    }
}