mod apple;
mod ctx;
mod game_object;
mod player;

use crate::game_object::GameObject;
use std::thread::sleep;
use std::time::{SystemTime, Duration};

fn main() {
    let mut player = player::Player::new();

    // let mut apple = apple::Apple::new();

    let mut ctx = ctx::Ctx::new(10, 10);

    loop {
        let current_time = SystemTime::now(); 
        sleep(Duration::new(0, 160_000_000));
        match current_time.elapsed() {
            Ok(_elapsed) => {
                // Update game objects
                player.update(&ctx);
                // apple.update(&ctx);

                // Render game objects
                player.render(&mut ctx);
                // apple.render(&mut ctx);
                ctx.render();
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
}
