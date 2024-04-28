use serde::{Deserialize, Serialize};
use crate::items::ItemMap;

#[derive(Debug, Clone)]
struct WidgetDetector {}

impl WidgetDetector {
    pub fn find(map: ItemMap) {

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedItem {
    color: Vec<u8>,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    status: String,
    items: Vec<DetectedItem>,
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_deserialize_response() {
        const RESPONSE: &str = r#"{ "status": "OK", "items": [{ "color": [254, 167, 0], "top": 696, "bottom": 734, "left": 871, "right": 891}, { "color": [226, 78, 27], "top": 697, "bottom": 741, "left": 748, "right": 768}, { "color": [0, 255, 0], "top": 546, "bottom": 670, "left": 350, "right": 371}, { "color": [0, 255, 0], "top": 406, "bottom": 444, "left": 886, "right": 906}, { "color": [0, 255, 0], "top": 406, "bottom": 513, "left": 404, "right": 424}, { "color": [0, 255, 0], "top": 356, "bottom": 461, "left": 405, "right": 425}, { "color": [0, 255, 0], "top": 306, "bottom": 370, "left": 404, "right": 424}, { "color": [127, 0, 127], "top": 486, "bottom": 608, "left": 812, "right": 835}, { "color": [255, 105, 180], "top": 772, "bottom": 836, "left": 43, "right": 65}, { "color": [255, 105, 180], "top": 442, "bottom": 493, "left": 44, "right": 65}, { "color": [255, 105, 180], "top": 392, "bottom": 489, "left": 43, "right": 63}, { "color": [255, 105, 180], "top": 343, "bottom": 389, "left": 43, "right": 62}, { "color": [255, 105, 180], "top": 292, "bottom": 352, "left": 44, "right": 65}, { "color": [255, 105, 180], "top": 195, "bottom": 243, "left": 44, "right": 60}, { "color": [228, 171, 183], "top": 242, "bottom": 285, "left": 43, "right": 63} ] }"#;
        let response: Response = serde_json::from_str(RESPONSE).unwrap();
        assert_eq!(response.status , "OK".to_string());
        assert_eq!(response.items.len() , 15);
    }

}


