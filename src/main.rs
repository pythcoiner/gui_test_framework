mod capture;
mod error;
mod graphical;
mod item;
mod liana_item;
mod liana_store;
mod ocr;
mod screenshot;
mod store;
mod widget_detector;

use crate::capture::Capture;
use crate::graphical::Color;
use crate::graphical::{CORAL, DARK_CYAN, HOT_PINK, LIME, MEDIUM_BLUE, ORANGE, PINK, PURPLE, RED};
use crate::liana_item::LianaItemType::{
    CheckBox, MenuButton, PrimaryButton, SecondaryButton, SettingSection, TextInput,
};
use error::Error;
use item::Item;
use liana_item::LianaItemType;
use screenshot::{ScreenShot, ScreenShotTrait};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn screenshot() -> Result<ScreenShot, Error> {
    let mut item_map: HashMap<Color, LianaItemType> = HashMap::new();
    item_map.insert(LIME, TextInput);
    item_map.insert(RED, PrimaryButton(true));
    item_map.insert(ORANGE, PrimaryButton(false));
    item_map.insert(PURPLE, SecondaryButton(true));
    item_map.insert(MEDIUM_BLUE, SecondaryButton(false));
    item_map.insert(HOT_PINK, MenuButton(false));
    item_map.insert(PINK, MenuButton(true));
    item_map.insert(CORAL, CheckBox);
    item_map.insert(DARK_CYAN, SettingSection);

    Capture::from_named_window("Liana", &item_map)
}

fn main() -> Result<(), Error> {
    let mut screen = screenshot()?;

    let btn = screen.find("Send", LianaItemType::MenuButton(true));
    btn.click();

    let mut screen = screenshot()?;

    let text = screen.find("Address", LianaItemType::TextInput);
    text.insert("bcrt1qyv9w4zx43kv96key52kwpkj3t9guwjflrz4vccftft02y4f0xtzqa65sfu");

    let text = screen.find("Payment label", LianaItemType::TextInput);
    text.insert("Maria paycheck");

    let text = screen.find("0.001 (in BTC)", LianaItemType::TextInput);
    text.insert("0.1");

    let text = screen.find("42 (in sats/vbyte)", LianaItemType::TextInput);
    text.insert("10");

    let btn = screen.find("Clear", LianaItemType::PrimaryButton(true));
    btn.click();

    Ok(())
}
