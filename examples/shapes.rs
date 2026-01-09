use graphicility::{Color, Config,};

fn main() {
    let conf = Config::builder().with_title("Geometry").build();

    graphicility::run_with(conf, |ctx| {
        let g= ctx.graphics();
        g.clear(Color::rgb(30, 30, 45)); // Dark navy background
        
        // 1. Lines: Drawing a simple grid or crosshair
        g.line((320, 0), (320, 400), Color::rgba(255, 255, 255, 0.2));
        g.line((0, 200), (640, 200), Color::rgba(255, 255, 255, 0.2));

        // 2. Rectangles: Outline vs standard blocks
        // Bottom left square
        g.rect((135, 260), (50, 50), Color::CYAN);
        
        // 3. Circles: Centered primitives
        // Drawing a target pattern
        let center = (160, 100);
        g.circle(center, 40, Color::RED);
        g.circle(center, 30, Color::WHITE);
        g.circle(center, 20, Color::RED);
        g.circle(center, 10, Color::WHITE);

        // 4. Triangles: Custom coordinates
        g.triangle(
            (475, 50),  // Top
            (525, 120), // Bottom Right
            (425, 120), // Bottom Left
            Color::YELLOW
        );
        
        g.triangle(
            (425, 250),  // Top
            (525, 310), // Bottom Right
            (425, 310), // Bottom Left
            Color::GREEN
        );

        // 6. Text Labels
        g.text((220, 10), "Graphicility Shapes Demo", Color::WHITE);
    });
}

