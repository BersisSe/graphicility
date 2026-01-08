use graphiclity::{Color, KeyCode};

fn main() {
    

    graphiclity::run(|ctx| {
       let (g, input) = ctx.split();
       g.clear(Color::WHITE);
       if input.key_pressed(KeyCode::Escape){
            println!("Key Espace is pressed");
       }
       if input.key_down(KeyCode::KeyW){
            println!("Key w is down");
       }
       let mpos = input.mouse_pos(); // Returns the Mouse position
       if let Some(pos) = mpos{
           g.text(pos, format!("X:{}, Y:{}", pos.0,pos.1), Color::RED);
       }

    });
}