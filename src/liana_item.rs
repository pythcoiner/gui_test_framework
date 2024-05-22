use crate::item::Item;
use crate::widget_detector::Position;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LianaItemType {
    TextInput,
    PrimaryButton(bool),
    SecondaryButton(bool),
    MenuButton(bool),
    CheckBox,
    SettingSection,
}

#[derive(Debug, Clone)]
pub struct LianaItem {
    pub text: Option<String>,
    pub position: Position,
    pub kind: LianaItemType,
}

impl Item<LianaItemType> for LianaItem {
    fn position(&self) -> Position {
        self.position.clone()
    }

    fn name(&self) -> String {
        self.text.clone().unwrap()
    }

    fn kind(&self) -> LianaItemType {
        self.kind
    }
}
