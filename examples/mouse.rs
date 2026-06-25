use graphicility::run;

fn main() {
    run(|ctx|{
        let input = ctx.input();
        
        if input.mouse_pressed(graphicility::MouseButton::Left){
            println!("adsa");
        }
    }).unwrap();
}