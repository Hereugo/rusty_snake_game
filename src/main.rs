mod food;
mod game;
mod player;

use game::Game;
use extra::rand::Randomizer;
use std::io::{stdout, Write};
use termion::{async_stdin, clear, cursor, style};
use termion::raw::IntoRawMode;

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = async_stdin();

    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    let mut game = Game {
        width: 80,
        height: 40,
        stdin,
        stdout,
        player: player::Player::new(80, 40),
        food: food::Food::new(80, 40),
        score: 0,
        speed: 0,
        rand: Randomizer::new(0),
    };

    game.start();

    write!(
        game.stdout,
        "{}{}{}",
        clear::All,
        style::Reset,
        cursor::Goto(1, 1)
    )
    .unwrap();
    game.stdout.flush().unwrap();
}
