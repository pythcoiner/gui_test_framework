use crate::widget_detector::Position;
use image::{GenericImageView, RgbaImage};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;
use std::fs;
use std::path::PathBuf;

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    // println!("path={:?}", abs_path);
    fs::read(abs_path)
}

#[allow(unused)]
pub struct Ocr {}

impl Ocr {
    pub fn build_engine() -> OcrEngine {
        // Use the `download-models.sh` script to download the models.
        let detection_model_data = read_file("ocrs/text-detection.rten").unwrap();
        let rec_model_data = read_file("ocrs/text-recognition.rten").unwrap();

        let detection_model = Model::load(&detection_model_data).unwrap();
        let recognition_model = Model::load(&rec_model_data).unwrap();

        OcrEngine::new(OcrEngineParams {
            recognition_model: Some(recognition_model),
            detection_model: Some(detection_model),
            ..Default::default()
        })
        .unwrap()
    }

    pub fn read_label(frame: &RgbaImage, position: &Position) -> Option<String> {
        let cropped: RgbaImage = frame
            .view(position.x, position.y, position.width, position.height)
            .to_image();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("cropped");
        path.push(format!("{}_{}.png", position.x, position.y));
        cropped.save(path).unwrap();

        // FIXME: engine should be created only once and passed as argument
        let engine = Self::build_engine();

        let img_source = ImageSource::from_bytes(&cropped, cropped.dimensions()).unwrap();
        let ocr_input = engine.prepare_input(img_source).unwrap();

        let word_rects = engine.detect_words(&ocr_input).unwrap();
        let line_rects = engine.find_text_lines(&ocr_input, &word_rects);
        let line_texts = engine.recognize_text(&ocr_input, &line_rects).unwrap();

        let lines: Vec<_> = line_texts
            .into_iter()
            .flatten()
            // Filter likely spurious detections. With future model improvements
            // this should become unnecessary.
            .filter_map(|l| {
                let s = l.to_string();
                if !s.is_empty() {
                    Some(s)
                } else {
                    None
                }
            })
            .collect();

        if !lines.is_empty() {
            let txt = lines[0].clone();
            println!("{}", txt);
            Some(txt)
        } else {
            println!("OCR failed!");
            None
        }
    }
}
