use graphicility::{Color, KeyCode, MouseButton};

fn main() {
    // Track points and point breaks
    let mut points: Vec<Option<(i32, i32)>> = vec![];

    graphicility::run(move |ctx| {
        let (g, input) = ctx.split();
        // Clear the screen
        g.clear(Color::rgb(20, 20, 20));

        // Add points while clicking
        if input.mouse_down(MouseButton::Left) {
            if let Some((mx, my)) = input.mouse_pos() {
                points.push(Some((mx as i32, my as i32)));
            }
        }
        // And add a point break when the mouse is released
        if input.mouse_released(MouseButton::Left) {
            points.push(None);
        }

        // Clear canvas with Space
        if input.key_down(KeyCode::Space) {
            points.clear();
        }

        // Draw the trail as lines this makes a better brush
        for pair in points.windows(2) {
            if let (Some(a), Some(b)) = (pair[0], pair[1]) {
                g.line(a, b, Color::YELLOW);
            }
        }

        g.text(
            (10, 10),
            "Left Click to Draw - Space to Clear",
            Color::WHITE,
        );
    })
    .unwrap();
}
