use qrcode::render::{Canvas, Pixel};

#[derive(Copy, Clone)]
struct BitPixel(bool);

struct BitCanvas {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Canvas for BitCanvas {
    type Pixel = BitPixel;
    type Image = Self;

    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        // todo fix 
        let default_value = 0; 

        let width = width.next_multiple_of(8) as usize;
        let height = height as usize;

        dbg!(width, height);
        let data = vec![default_value; width * height / 8];

        Self {
            data,
            width,
            height,
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        // self.data[y * 4 + (x / 8)] |= 1 << (7 - (x % 8));
        let index = y as usize * (self.width / 8) + (x as usize / 8);
        self.data[index] |= 1 << (7 - x % 8);
    }

    fn into_image(self) -> Self::Image {
        self
    }

}

impl Pixel for BitPixel {
    type Image = BitCanvas;
    type Canvas = BitCanvas;

    fn default_color(color: qrcode::Color) -> Self {
        match color {
            qrcode::Color::Light => Self(true),
            qrcode::Color::Dark => Self(false),
        }
    }
}
