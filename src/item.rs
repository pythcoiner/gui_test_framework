use autopilot::geometry::Rect;

#[allow(unused)]
fn click_mouse() {
    autopilot::mouse::click(autopilot::mouse::Button::Left, None);
}
#[allow(unused)]
fn move_mouse(pos: Rect) {
    autopilot::mouse::move_to(autopilot::geometry::Point {
        x: (pos.origin.x + pos.size.width / 2.0),
        y: (pos.origin.y + pos.size.height / 2.0),
    })
    .unwrap();
}
#[allow(unused)]
fn input_text(text: String) {
    autopilot::key::type_string(&text, &[], 30.0, 0.0);
}
#[allow(unused)]
pub trait Item<K> {
    // To be reimplemented
    fn position(&self) -> Rect;
    fn name(&self) -> String;
    fn kind(&self) -> K;

    // To not be reimplemented
    fn click(&self) {
        self.hover();
        click_mouse();
    }
    fn insert(&self, text: String) {
        self.click();
        input_text(text);
    }
    fn hover(&self) {
        let position = self.position();
        move_mouse(position);
    }
}
