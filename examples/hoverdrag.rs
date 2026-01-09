use graphicility::{Color, Vec2};

fn main() {
    // Typed Vectors
    let mut box_pos = Vec2 { x: 100, y: 100 };
    let box_size = Vec2 { x: 50, y: 50 };

    graphicility::run( move |ctx| {
        let (g, input) = ctx.split();
        // Clear screen to black
        g.clear(Color::BLACK);

        // Map mouse manually if mouse_pos() is still giving physical coords
        let mouse = input.mouse_pos();

        let mut color = Color::BLUE;

        if let Some(m) = mouse {
            //Some Collision detection
            let x = m.0.floor() as i32;
            let y = m.1.floor() as i32;
            if x >= box_pos.x && x <= box_pos.x + box_size.x &&
               y >= box_pos.y && y <= box_pos.y + box_size.y {
                color = Color::RED;
                
                // If clicking, drag the box!
                if input.mouse_down(graphicility::MouseButton::Left) {
                    box_pos.x = x - box_size.x / 2;
                    box_pos.y = y - box_size.y / 2;
                }
            }
        }

        g.rect(box_pos, box_size, color);
        g.text((10, 10), "Hover to highlight, Click to drag", Color::WHITE);
    });
}