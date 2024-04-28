use std::collections::HashMap;
use autopilot::geometry::Rect;
use crate::graphical::Color;

pub type ItemMap = HashMap<Color, ItemKind>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    TextInput,
    PrimaryButton(bool),
    SecondaryButton(bool),
    MenuButton(bool),
    CheckBox,
    SettingSection,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Item {
    pub text: String,
    pub position: Rect,
    pub kind: Option<ItemKind>,
}

