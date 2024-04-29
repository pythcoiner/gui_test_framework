use autopilot::bitmap::Bitmap;
use autopilot::geometry::Point;
use rusty_tesseract::image::{Rgba, RgbaImage};
use crate::Error;

use image::Rgba as ImageRgba;
use rusty_tesseract::image::Rgba as TessRgba;

pub trait Graphical {
    fn from_bitmap(bitmap: &Bitmap) -> Result<Box<Self>, Error>;
    fn draw_rectangle(&mut self, xa: u32, xb: u32, ya: u32, yb: u32, color: Rgba<u8>);
    fn filter_by_color(&self, color: Color) -> Self;
}

impl Graphical for RgbaImage {
    fn from_bitmap(bitmap: &Bitmap) -> Result<Box<Self>, Error> {
        fn convert_rgba(image_rgba: ImageRgba<u8>) -> TessRgba<u8> {
            TessRgba([image_rgba.0[0], image_rgba.0[1], image_rgba.0[2], image_rgba.0[3]])
        }

        let (width, height) = (bitmap.size.width as u32, bitmap.size.height as u32);
        let mut imgbuf = RgbaImage::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pos = Point::new(x as f64, y as f64);
            *pixel = convert_rgba(bitmap.get_pixel(pos));
        }

        Ok(Box::new(imgbuf))
    }

    fn draw_rectangle(&mut self, mut xa: u32, mut xb: u32, mut ya: u32, mut yb: u32, color: Rgba<u8>) {
        // TODO: check boundaries
        if yb < ya {
            (ya, yb) = (yb, ya);
        }
        if xb < xa {
            (xa, xb) = (xb, xa);
        }
        for x in xa..xb {
            for y in ya..yb {
                self.put_pixel(x, y, color);
            }
        }
    }

    fn filter_by_color(&self, color: Color) -> Self {
        
        fn color_match(this: Rgba<u8>, other: Rgba<u8>) -> bool{
            let trigger = 80i16;
            
            #[allow(clippy::if_same_then_else, clippy::needless_bool)]
            if (this.0[0] as i16 - other.0[0] as i16).abs() > trigger {false}
            else if (this.0[1] as i16 - other.0[1] as i16).abs() > trigger {false}
            else if (this.0[2] as i16 - other.0[2] as i16).abs() > trigger {false}
            else {true}
        }
        
        let mut image = self.clone();
        image.enumerate_pixels_mut()
            .for_each(|(_, _, pixel)| {
                if !color_match(*pixel , color.into()) {
                    *pixel = Rgba([255, 255, 255, 255]); // Set to white
                } else {
                    *pixel = Rgba([0, 0, 0, 255]); // Set to black
                }
            });
        image
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    const fn new(r: u8, g: u8, b: u8) ->  Self {
        Color {r, g, b}
    }
}

impl From<Color> for Rgba<u8> {
    fn from(value: Color) -> Self {
        Rgba([value.r, value.g, value.b, 255])
    }
}

pub const RED: Color = Color::new(0xe2, 0x4e, 0x1b);
pub const ORANGE: Color = Color::new(0xff, 0xa7, 0x00);
pub const MEDIUM_BLUE: Color = Color::new(0x00, 0x00, 0xcd);
pub const DARK_CYAN: Color = Color::new(0x00, 0x8b, 0x8b);
pub const LIME: Color = Color::new(0x00, 0xff, 0x00);
pub const HOT_PINK: Color = Color::new(0xff, 0x69, 0xb4);
pub const PINK: Color = Color::new(0xff, 0xc0, 0xcb);
#[allow(unused)]
pub const WHEAT: Color = Color::new(0xf5, 0xde, 0xb3);
pub const PURPLE: Color = Color::new(0x80, 0x00, 0x80);
pub const CORAL: Color = Color::new(0xff, 0x7f, 0x50);
