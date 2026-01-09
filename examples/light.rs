    use graphicility::{Color, Config};

    fn main() {
        let conf = Config::default();

        graphicility::run_with(conf, |ctx| {
            let (g, input) = ctx.split();
            g.clear(Color::BLACK);

            // Get mouse position (scaled to logical coordinates)
            if let Some((mx, my)) = input.mouse_pos() {
                let mouse_pos = (mx as i32, my as i32);
                
                // Draw a "glow" effect with nested circles
                g.circle(mouse_pos, 40, Color::rgba(0, 255, 255, 0.3));
                g.circle(mouse_pos, 20, Color::rgba(0, 255, 255, 0.6));
                g.circle(mouse_pos, 5, Color::CYAN);
                
                g.text((mx as i32 + 10, my as i32 + 10), format!("X: {} Y: {}", mx, my), Color::WHITE);
            }

            g.text((10, 10), "Move mouse to shine the light", Color::WHITE);
        });
    }