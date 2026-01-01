use graphiclity::{Color, Config};

fn main() {
    let conf = Config {
        resizeable: false,
        logical_width: 320,
        logical_height: 240,
        window_width: 640,
        window_height: 480,
        ..Default::default()
    };
    
    let mut x: i32 = 50;
    let mut y: i32 = 50;
    let mut vx: i32 = 2;
    let mut vy: i32 = 3;
    let rect_w: i32 = 20;
    let rect_h: i32 = 20;
    
    graphiclity::run_with(conf, move |g| {
        let (width, height) = g.logical_size();
        
        g.clear(Color::WHITE);
        
        // Draw title
        g.text(10, 10, "Bouncing Rectangle Demo", Color::CYAN);
        
        // Draw coordinates
        g.text(10, 220, &format!("X: {} Y: {}", x, y), Color::BLACK);
        
        x += vx;
        y += vy;
        
        if x <= 0 || x + rect_w >= width as i32 {
            vx = -vx;
        }
        if y <= 0 || y + rect_h >= height as i32 {
            vy = -vy;
        }
        
        g.rect(x, y, rect_w, rect_h, Color::rgb(128, 23, 255));
    });
}