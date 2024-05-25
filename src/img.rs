use image::{ImageBuffer, Rgba};

pub struct CodeImg {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    module_size: u32,
    black: Rgba<u8>,
    white: Rgba<u8>,
    reserved: Rgba<u8>,
}

impl CodeImg {
    // true = black, false = white
    pub fn image(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.img
    }
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
        width: u32,
        height: u32,
        black: Rgba<u8>,
        white: Rgba<u8>,
        reserved: Rgba<u8>,
    ) -> Self {
        let mut code = CodeImg {
            img: ImageBuffer::new(width * module_size, height * module_size),
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
                code.fill_module(x, (height - 1) - y, color);
                code.fill_module((width - 1) - x, y, color);
            }
        }

        // add alignment patterns
        for x in 0..5 {
            for y in 0..5 {
                let color = !(((x == 1 || x == 3) && (y >= 1 && y <= 3))
                    || ((y == 1 || y == 3) && (x >= 1 && x <= 3)));
                code.fill_module(32 + x, 32 + y, color);
            }
        }

        for i in 8..(width - 8) {
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
                    code.fill_module(8, (height - 1) - i, true);
                } else {
                    code.reserve(8, (height - 1) - i);
                };
                code.reserve((width - 1) - i, 8);
            }
        }

        // reserve version information area
        // for i in 0..3 {
        //     for j in 0..6 {
        //         fill_module(j, ((width - 1) - 8) - i, reserved, &mut code);
        //         fill_module(((height - 1) - 8) - i, j, reserved, &mut code);
        //     }
        // }

        code
    }
}
