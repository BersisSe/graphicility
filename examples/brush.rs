use graphicility::{Color, MouseButton, KeyCode};

fn main() {
    let mut points: Vec<(i32, i32)> = vec![];

    graphicility::run( move |ctx| {
        let (g, input) = ctx.split();
        // Clear the screen
        g.clear(Color::rgb(20, 20, 20));

        // Add points while clicking
        if input.mouse_down(MouseButton::Left) {
            if let Some((mx, my)) = input.mouse_pos() {
                points.push((mx as i32, my as i32));
            }
        }

        // Clear canvas with Space
        if input.key_down(KeyCode::Space) {
            points.clear();
        }

        // Draw the trail
        for p in &points {
            g.pixel(*p, Color::YELLOW);
        }

        g.text((10, 10), "Left Click to Draw | Space to Clear", Color::WHITE);
    });
}