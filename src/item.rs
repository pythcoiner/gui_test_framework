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
fn draw_mouse(pos: Rect) {
    autopilot::mouse::smooth_move(
        autopilot::geometry::Point {
            x: pos.origin.x,
            y: pos.origin.y,
        },
        Some(0.3),
    );
    autopilot::mouse::smooth_move(
        autopilot::geometry::Point {
            x: pos.origin.x + pos.size.width,
            y: pos.origin.y,
        },
        Some(0.3),
    );
    autopilot::mouse::smooth_move(
        autopilot::geometry::Point {
            x: pos.origin.x + pos.size.width,
            y: pos.origin.y + pos.size.height,
        },
        Some(0.3),
    );
    autopilot::mouse::smooth_move(
        autopilot::geometry::Point {
            x: pos.origin.x,
            y: pos.origin.y + pos.size.height,
        },
        Some(0.3),
    );
    autopilot::mouse::smooth_move(
        autopilot::geometry::Point {
            x: pos.origin.x,
            y: pos.origin.y,
        },
        Some(0.3),
    );
}

#[allow(unused)]
fn input_text(text: String) {
    autopilot::key::type_string(&text, &[], 80.0, 0.0);
}
#[allow(unused)]
pub trait Item<K> {
    // To be reimplemented
    fn position(&self) -> Rect;
    fn name(&self) -> String;
    fn kind(&self) -> K;
    fn set_position(&mut self, position: Rect);
    fn set_name(&mut self, name: String);

    // To not be reimplemented
    fn click(&self) {
        self.hover();
        click_mouse();
    }
    fn insert(&self, text: &str) {
        self.click();
        input_text(text.to_string());
    }
    fn hover(&self) {
        let position = self.position();
        move_mouse(position);
    }
    fn draw(&self) {
        draw_mouse(self.position());
    }
}
