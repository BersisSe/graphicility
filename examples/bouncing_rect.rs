use graphicility::{Color, Config, Vec2};

fn main() {
    let conf = Config::builder()
        .with_title("Bouncing Rect")
        .resizeable(false)
        .with_target_fps(60)
        .build();

    // Using 2D Vectors for Typed Geometric Data.
    let mut pos = Vec2 { x: 50, y: 50 };
    let mut vel = Vec2 { x: 2, y: 3 };
    let size = Vec2 { x: 20, y: 20 };

    graphicility::run_with(conf, move |ctx| {
        let dt = ctx.delta_time();
        let g = ctx.graphics();

        // We multiply by 60.0 so that '2' feels like '2 pixels per frame' at 60fps
        pos.x += (vel.x as f64 * dt * 60.0) as i32;
        pos.y += (vel.y as f64 * dt * 60.0) as i32;

        let (width, height) = g.logical_size();

        // Collision detection
        if pos.x <= 0 || pos.x + size.x >= width as i32 {
            vel.x = -vel.x;
        }
        if pos.y <= 0 || pos.y + size.y >= height as i32 {
            vel.y = -vel.y;
        }

        // Drawing
        g.clear(Color::WHITE);
        
        // Notice we can pass 'pos' and 'size' directly or as tuples. Painting methods accept Into<Vec2>
        g.rect(pos, size, Color::rgb(128, 23, 255));

        // Lastly, lest draw some UI
        g.text((10, 10), "Graphicility v0.3.0", Color::CYAN);
        g.text((10, height as i32 - 20), format!("Pos: {}, {}", pos.x, pos.y), Color::BLACK);
        
    }).unwrap();
}