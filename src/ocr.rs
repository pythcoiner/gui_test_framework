use crate::error::Error;
use crate::items::{Item, ItemKind};
use crate::ScreenShot;
use std::fs;
use std::path::PathBuf;
use image::{GenericImageView, RgbaImage};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use crate::widget_detector::Position;

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    fs::read(abs_path)
}

#[allow(unused)]
pub struct Ocr{}

impl Ocr {
    pub fn build_engine() -> OcrEngine {
        // Use the `download-models.sh` script to download the models.
        // let detection_model_data = include_bytes!("ocrs/text-detection.rten");
        let rec_model_data = read_file("ocrs/text-recognition.rten").unwrap();

        // let detection_model = Model::load(detection_model_data).unwrap();
        let recognition_model = Model::load(&rec_model_data).unwrap();

        OcrEngine::new(OcrEngineParams {
            recognition_model: Some(recognition_model),
            ..Default::default()
        }).unwrap()
    }


    pub fn read_label(frame: &RgbaImage, position: &Position) -> Option<String> {

        let cropped: RgbaImage = frame.view(
            position.x,
            position.y,
            position.width,
            position.height)
            .to_image();

        // FIXME: engine should be created only once and passed as argument
        let engine = Self::build_engine();

        let img_source = ImageSource::from_bytes(&frame, frame.dimensions()).unwrap();
        let ocr_input = engine.prepare_input(img_source).unwrap();

        if let Ok(txt) = engine.get_text(&ocr_input) {
            Some(txt)
        } else {
            None
        }



    }

}

