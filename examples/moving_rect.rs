use graphiclity::{Color, KeyCode};

fn main() {
    let mut x = 100.0;
    let mut y = 100.0;

    graphiclity::run(|ctx| {
        let dt = ctx.delta_time(); // Getthe delta time. its important to call this before `split`
        let (g, input) = ctx.split();

        
        if input.key_down(KeyCode::KeyW) { y -= 150.0 * dt; } // Move the recteangle using 150 speed and delta
        if input.key_down(KeyCode::KeyS) { y += 150.0 * dt; }
        if input.key_down(KeyCode::KeyA) { x -= 150.0 * dt; }
        if input.key_down(KeyCode::KeyD) { x += 150.0 * dt; }

        // 2. Dash (Single Trigger - now works perfectly!)
        if input.key_pressed(KeyCode::Space) {
            x += 40.0; 
        }

        g.clear(Color::BLACK);
        g.rect((x as i32, y as i32), (20, 20), Color::CYAN);
        g.text((10, 10), "WASD to move, Space to Dash", Color::WHITE);
    });
}