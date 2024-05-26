use image::{ImageBuffer, Rgba};

use crate::consts;

pub struct CodeImg {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    module_size: u32,
    black: Rgba<u8>,
    white: Rgba<u8>,
    reserved: Rgba<u8>,
}

impl CodeImg {
    pub fn image(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.img
    }

    pub fn save(&self) {
        self.img.save("template.png");
    }

    // true = black, false = white
    pub fn fill_module(&mut self, mx: u32, my: u32, val: bool) {
        let color = if val { self.black } else { self.white };
        for px in 0..self.module_size {
            for py in 0..self.module_size {
                self.img.put_pixel(
                    mx * self.module_size + px,
                    my * self.module_size + py,
                    color,
                );
            }
        }
    }
    pub fn debug(&mut self, mx: u32, my: u32, color: Rgba<u8>) {
        for px in 0..self.module_size {
            for py in 0..self.module_size {
                if px != 0 && px != self.module_size - 1 && py != 0 && py != self.module_size - 1 {
                    self.img.put_pixel(
                        mx * self.module_size + px,
                        my * self.module_size + py,
                        color,
                    );
                } else {
                    self.img.put_pixel(
                        mx * self.module_size + px,
                        my * self.module_size + py,
                        self.white,
                    );
                }
            }
        }
    }
    fn reserve(&mut self, mx: u32, my: u32) {
        for px in 0..self.module_size {
            for py in 0..self.module_size {
                self.img.put_pixel(
                    mx * self.module_size + px,
                    my * self.module_size + py,
                    self.reserved,
                );
            }
        }
    }
    pub fn is_open(&self, mx: u32, my: u32) -> bool {
        if let Some(p) = self
            .img
            .get_pixel_checked(mx * self.module_size, my * self.module_size)
        {
            if p[3] == 0 {
                return true;
            }
        }
        false
    }
    pub fn is_reserved(&self, mx: u32, my: u32) -> bool {
        if let Some(p) = self
            .img
            .get_pixel_checked(mx * self.module_size, my * self.module_size)
        {
            if *p == self.reserved {
                return true;
            }
        }
        false
    }
    pub fn new(
        module_size: u32,
        side_length: u32,
        black: Rgba<u8>,
        white: Rgba<u8>,
        reserved: Rgba<u8>,
        version: u8,
    ) -> Self {
        let mut code = CodeImg {
            img: ImageBuffer::new(side_length * module_size, side_length * module_size),
            module_size,
            black,
            white,
            reserved,
        };

        // add finder patterns + separators
        for x in 0..8 {
            for y in 0..8 {
                let color = !(((x == 1 || x == 5) && (y >= 1 && y <= 5))
                    || ((y == 1 || y == 5) && (x >= 1 && x <= 5))
                    || x == 7
                    || y == 7);

                code.fill_module(x, y, color);
                code.fill_module(x, (side_length - 1) - y, color);
                code.fill_module((side_length - 1) - x, y, color);
            }
        }

        // add alignment patterns

        if version > 1 {
            let (distance, number) = consts::pattern_locations(version);

            let patterns_iter = (0..number).map(|x| x * distance + 6);

            for col in patterns_iter.clone() {
                for row in patterns_iter.clone() {
                    if code.is_open(col, row) {
                        for x in 0..5 {
                            for y in 0..5 {
                                let color = !(((x == 1 || x == 3) && (y >= 1 && y <= 3))
                                    || ((y == 1 || y == 3) && (x >= 1 && x <= 3)));
                                code.fill_module((col - 2) + x, (row - 2) + y, color);
                            }
                        }
                    }
                }
            }
        }

        for i in 8..(side_length - 8) {
            let color = i % 2 == 0;
            if code.is_open(i, 6) {
                code.fill_module(i, 6, color);
                code.fill_module(6, i, color);
            }
        }

        // add dark module and reserved areas
        for i in 0..9 {
            if code.is_open(i, 8) {
                code.reserve(i, 8);
                code.reserve(8, i);
            }
            if i != 8 {
                if i == 7 {
                    code.fill_module(8, (side_length - 1) - i, true);
                } else {
                    code.reserve(8, (side_length - 1) - i);
                };
                code.reserve((side_length - 1) - i, 8);
            }
        }

        // reserve version information area if applicable
        if version > 6 {
            for i in 0..3 {
                for j in 0..6 {
                    code.reserve(j, ((side_length - 1) - 8) - i);
                    code.reserve(((side_length - 1) - 8) - i, j);
                }
            }
        }

        // code.img.save("template.png").unwrap();
        // panic!("stopped.");

        code
    }
}
