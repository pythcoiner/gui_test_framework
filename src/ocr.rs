use std::collections::HashMap;
use imageproc::definitions::HasBlack;
use rusty_tesseract::{Args, image_to_boxes};
use rusty_tesseract::image::{DynamicImage, GenericImageView, RgbaImage, Luma, ImageBuffer, imageops::overlay};
use rusty_tesseract::tesseract::input::Image as TessImage;
use crate::widget_detector::Position;

pub type GreyImage = ImageBuffer<Luma<u8>, Vec<u8>>;

#[allow(unused)]
pub struct Ocr{}

impl Ocr {

    fn extend_border(img: &GreyImage, border_size: u32) -> GreyImage {
        // Calculate the new dimensions
        let new_width = img.width()  + 2 * border_size;
        let new_height = img.height()  + 2 * border_size;

        let black: Luma<u8> = Luma::black();
        // Create a new image with the increased dimensions and fill it with the border color
        let mut new_img = ImageBuffer::from_pixel(new_width, new_height, black);

        // Copy the original image onto the new image at the specified offset
        overlay(&mut new_img, img, border_size as i64, border_size as i64);

        new_img
    }
    pub fn read_label(frame: &RgbaImage, position: &Position) -> Option<String> {
        // println!("Ocr::read_label() => Image {}x{}", frame.width(), frame.height());
        // println!("Ocr::read_label() => Position [{}:{}, {}:{}]",
        //          position.x,
        //          position.y,
        //          position.x + position.width,
        //          position.y + position.height
        //
        // );

        let cropped: RgbaImage = frame.view(
            position.x,
            position.y,
            position.width,
            position.height)
            .to_image();

        let gray_img = GreyImage::from(DynamicImage::ImageRgba8(cropped));

        // let gray_img = imageproc::contrast::adaptive_threshold(&gray_img, 2);

        // let gray_img = Self::extend_border(&gray_img, 50);

        gray_img.save("gray_img.png").unwrap();

        let dynamic_image = DynamicImage::ImageLuma8(gray_img);
        // println!("Dynamic image");

        let tess_image = if let Ok(img) = TessImage::from_dynamic_image(&dynamic_image) {
            img
        } else {
            return None
        };
        
        // println!("Tess image");

        let args = Args {
            lang: "eng".into(),
            config_variables: HashMap::from([(
                "tessedit_char_whitelist".into(),
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.0123456789/()".into(),
            )]),
            // 15" screen 1920x1080 = 140DPI
            dpi: Some(140),
            // Page segmentation modes (PSM):
            // 0    Orientation and script detection (OSD) only.
            // 1    Automatic page segmentation with OSD.
            // 2    Automatic page segmentation, but no OSD, or OCR.
            // 3    Fully automatic page segmentation, but no OSD. (Default)
            // 4    Assume a single column of text of variable sizes.
            // 5    Assume a single uniform block of vertically aligned text.
            // 6    Assume a single uniform block of text.
            // 7    Treat the image as a single text line.
            // 8    Treat the image as a single word.
            // 9    Treat the image as a single word in a circle.
            // 10    Treat the image as a single character.
            // 11    Sparse text. Find as much text as possible in no particular order.
            // 12    Sparse text with OSD.
            // 13    Raw line. Treat the image as a single text line, bypassing hacks that are Tesseract-specific.
            psm: Some(7),
            // OCR Engine modes: (see https://github.com/tesseract-ocr/tesseract/wiki#linux)
            // 0    Legacy engine only.
            // 1    Neural nets LSTM engine only.
            // 2    Legacy + LSTM engines.
            // 3    Default, based on what is available.
            oem: Some(3),
        };

        // OCR Recognition
        // TODO: lets go back to image_to_boxes() + sorting the chars
        if let Ok(ocr) = image_to_boxes(&tess_image, &args) {
            let mut boxes = ocr.boxes;
            boxes.sort_by(|a,b| {
                    let a = a.left + a.right;
                    let b = b.left + b.right;
                    a.cmp(&b)
                });
            let boxes = boxes
                .into_iter()
                .map(|b| b.symbol)
                .collect::<Vec<_>>();
            let ocr = boxes.join("");
            println!("ocr={:?}", ocr);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            Some(ocr.to_string())
        } else { None }
        
    }

}

