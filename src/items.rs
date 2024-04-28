use std::collections::HashMap;
use crate::graphical::Color;
use crate::widget_detector::Position;

pub type ItemMap = HashMap<Color, ItemKind>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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
    pub text: Option<String>,
    pub position: Position,
    pub kind: ItemKind,
}

