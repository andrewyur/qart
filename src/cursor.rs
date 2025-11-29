use anyhow::anyhow;

// abstracts navigating the qr code when placing modules
use crate::img::CodeImg;

pub struct Cursor<'a> {
    // these should only be changed by the struct itsself
    pub x: u32,
    pub y: u32,
    next_move: Move,
    prev_move: Move,
    code: &'a mut CodeImg,
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    UpRight,
    DownRight,
}

impl<'a> Cursor<'a> {
    pub fn new(code: &'a mut CodeImg, side_len: u32) -> Self {
        Self {
            x: side_len - 1,
            y: side_len - 1,
            next_move: Move::Left,
            prev_move: Move::UpRight,
            code,
        }
    }
    pub fn next(&mut self) -> anyhow::Result<bool> {
        // TODO: This code is ugly and unintuitive, see https://www.pclviewer.com/rs2/qrtopology.htm
        match self.next_move {
            Move::Left => {
                if self.x != 0 && !self.code.is_open(self.x - 1, self.y) {
                    self.code.save()?;
                    return Err(anyhow!("No valid moves! at ({},{})", self.x, self.y));
                }
                self.x -= 1;
                match &self.prev_move {
                    Move::Left => {
                        if self.y != 0 && self.code.is_open(self.x + 1, self.y - 1) {
                            self.next_move = Move::UpRight;
                        } else {
                            self.next_move = Move::DownRight;
                        }
                        self.prev_move = Move::Left;
                    }
                    other_move => {
                        self.next_move = *other_move;
                        self.prev_move = Move::Left;
                    }
                }
            }
            Move::UpRight => {
                if self.y != 0 && self.code.is_open(self.x + 1, self.y - 1) {
                    self.x += 1;
                    self.y -= 1;
                    self.next_move = Move::Left;
                    self.prev_move = Move::UpRight;
                } else if self.y >= 1 && self.code.is_open(self.x, self.y - 1) {
                    self.y -= 1;
                    self.next_move = Move::UpRight;
                    self.prev_move = Move::UpRight;
                } else if self.y >= 2 && self.code.is_open(self.x + 1, self.y - 2) {
                    self.x += 1;
                    self.y -= 2;
                    self.next_move = Move::Left;
                    self.prev_move = Move::UpRight;
                } else if self.y >= 2 && self.code.is_open(self.x, self.y - 2) {
                    self.y -= 2;
                    self.next_move = Move::UpRight;
                    self.prev_move = Move::UpRight;
                } else if self.y >= 6 && self.code.is_open(self.x + 1, self.y - 6) {
                    self.x += 1;
                    self.y -= 6;
                    self.next_move = Move::Left;
                    self.prev_move = Move::UpRight;
                } else if self.y >= 7 && self.x >= 2 && self.code.is_open(self.x - 2, self.y - 7) {
                    self.x -= 2;
                    self.y -= 7;
                    self.next_move = Move::DownRight;
                    self.prev_move = Move::DownRight;
                } else if self.x >= 1 && self.code.is_open(self.x - 1, self.y) {
                    self.x -= 1;
                    self.next_move = Move::Left;
                    self.prev_move = Move::Left;
                } else if self.x >= 2 && self.code.is_open(self.x - 2, self.y) {
                    self.x -= 2;
                    self.next_move = Move::Left;
                    self.prev_move = Move::Left;
                } else {
                    self.code.save()?;
                    return Err(anyhow!("No valid moves! at ({},{})", self.x, self.y));
                }
            }
            Move::DownRight => {
                if self.code.is_open(self.x + 1, self.y + 1) {
                    self.x += 1;
                    self.y += 1;
                    self.next_move = Move::Left;
                    self.prev_move = Move::DownRight;
                } else if self.code.is_open(self.x, self.y + 1) {
                    self.y += 1;
                    self.prev_move = Move::DownRight;
                    self.next_move = Move::DownRight;
                } else if self.code.is_open(self.x + 1, self.y + 2) {
                    self.x += 1;
                    self.y += 2;
                    self.next_move = Move::Left;
                    self.prev_move = Move::DownRight;
                } else if self.code.is_open(self.x, self.y + 2) {
                    self.y += 2;
                    self.prev_move = Move::DownRight;
                    self.next_move = Move::DownRight;
                } else if self.code.is_open(self.x + 1, self.y + 6) {
                    self.x += 1;
                    self.y += 6;
                    self.next_move = Move::Left;
                    self.prev_move = Move::DownRight;
                } else if self.x >= 1 && self.code.is_open(self.x - 1, self.y) {
                    self.x -= 1;
                    self.next_move = Move::Left;
                    self.prev_move = Move::Left;
                } else if self.x >= 1 && self.y >= 8 && self.code.is_open(self.x - 1, self.y - 8) {
                    self.x -= 1;
                    self.y -= 8;
                    self.next_move = Move::Left;
                    self.prev_move = Move::Left;
                } else {
                    return Ok(false);
                }
            }
        };
        return Ok(true);
    }
    pub fn place(&mut self, color: bool) {
        self.code.fill_module(self.x, self.y, color)
    }
    pub fn place_debug(&mut self, color: image::Rgba<u8>) {
        self.code.debug(self.x, self.y, color)
    }
}
