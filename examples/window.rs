use graphicility::{Color, Window};

fn main() {
    // Create a window
    let mut win = Window::new(Default::default()).unwrap();
    // Create our loop
    while win.is_running() {
        // Get the frame
        let frame = win.next_frame();
        // Draw
        frame.graphics().rect((5, 5), (10, 10), Color::RED);
        // And send the frame for rendering 
        win.end_frame();
    }
}