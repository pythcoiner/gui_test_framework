use std::collections::HashMap;

use crate::graphical::Color;
use crate::graphical::Graphical;
use crate::item::Item;
use crate::liana_item::{LianaItem, LianaItemType};
use crate::liana_store::LianaStore;
use crate::store::Store;
use autopilot::geometry::Rect;
use image::{Rgba, RgbaImage};

#[derive(Debug)]
pub struct ScreenShot {
    pub frame: RgbaImage,
    pub image: Option<RgbaImage>,
    pub position: Rect,
    pub item_map: HashMap<Color, LianaItemType>,
    pub store: LianaStore,
}

#[allow(unused)]
impl ScreenShotTrait<LianaItem, LianaItemType, LianaStore> for ScreenShot {
    fn position(&self) -> Rect {
        self.position
    }

    fn store(&mut self, kind: LianaItemType) -> &mut Vec<LianaItem> {
        self.store.bucket(kind)
    }

    fn image(&self) -> Option<RgbaImage> {
        self.image.clone()
    }
}

#[allow(unused)]
pub trait ScreenShotTrait<I, K, S>
where
    I: Item<K>,
    K: PartialEq + Copy,
    S: Store<I, K>,
{
    fn position(&self) -> Rect;

    fn store(&mut self, kind: K) -> &mut Vec<I>;

    fn image(&self) -> Option<RgbaImage>;

    fn append_items(&mut self, items: Vec<I>) {
        for item in items {
            self.push(item);
        }
    }

    fn push(&mut self, mut item: I) {
        // Adjust item's position by adding the offset of the screenshot
        item.position().origin.x = self.position().origin.x - item.position().origin.x;
        item.position().origin.y = -(self.position().origin.y - item.position().origin.y);
        item.position().size.height = -item.position().size.height;
        let kind = item.kind();
        // Append the adjusted item to the items vector
        self.store(kind).push(item);
    }

    fn draw_item_box(&mut self, rect: &Rect, b: u32, color: Rgba<u8>) {
        let h = self.position().size.height as u32;
        if let Some(img) = &mut self.image() {
            let xa: u32 = rect.origin.x as u32;
            let xb: u32 = (rect.origin.x + rect.size.width) as u32;
            let ya: u32 = h - (rect.origin.y as u32);
            let yb: u32 = h - ((rect.origin.y + rect.size.height) as u32);

            img.draw_rectangle(xa - b, xb + b, ya - b, ya, color);
            img.draw_rectangle(xa - b, xb + b, yb, yb + b, color);
            img.draw_rectangle(xa - b, xa, ya, yb, color);
            img.draw_rectangle(xb, xb + b, ya, yb, color);
        }
    }

    fn find(&mut self, name: &str, kind: K) -> &mut I {
        let store = self.store(kind);
        let item = store.iter_mut().find(|i| i.name() == name);
        item.unwrap()
    }
}
