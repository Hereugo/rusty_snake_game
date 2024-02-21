use crate::food::Food;
use crate::player::{Direction, Player};
use extra::rand::Randomizer;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::{clear, color, cursor, style};

pub struct Game<R, W> {
    pub width: u16,
    pub height: u16,
    pub stdin: R,
    pub stdout: W,
    pub player: Player,
    pub food: Food,
    pub score: u64,
    pub speed: u64,
    pub rand: Randomizer,
}

impl<R: Read, W: Write> Game<R, W> {
    pub fn start(&mut self) {
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        self.reset();

        let mut before = Instant::now();
        'game_loop: loop {
            let interval = 1000 / self.speed;
            let now = Instant::now();
            let dt = (now.duration_since(before).subsec_nanos() / 1_000_000) as u64;

            if dt < interval {
                sleep(Duration::from_millis(interval - dt));
                continue;
            }

            before = now;

            if !self.handle_input() {
                break 'game_loop;
            }

            self.update();

            if self.check_game_over() {
                if self.game_over() {
                    self.reset();
                    continue;
                }

                break 'game_loop;
            }

            self.draw();

            write!(self.stdout, "{}", style::Reset).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn handle_input(&mut self) -> bool {
        let mut buffer = [0];
        self.stdin.read(&mut buffer).unwrap();

        self.rand.write_u8(buffer[0]);

        match buffer[0] {
            b'q' => return false,
            b'i' | b'w' => self.player.turn(Direction::Up),
            b'j' | b'a' => self.player.turn(Direction::Left),
            b'k' | b's' => self.player.turn(Direction::Down),
            b'l' | b'd' => self.player.turn(Direction::Right),
            _ => {}
        }

        true
    }

    fn game_over(&mut self) -> bool {
        pub const GAME_OVER: [&str; 5] = [
            "╔═════════════════╗",
            "║───┬GAME OVER────║",
            "║ r │ replay      ║",
            "║ q │ quit        ║",
            "╚═══╧═════════════╝",
        ];

        for (i, txt) in GAME_OVER.iter().enumerate() {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto((self.width - 19) / 2, (self.height - 5) / 2 + i as u16),
                txt
            )
            .unwrap();
        }

        self.stdout.flush().unwrap();

        loop {
            let mut buffer = [0];
            self.stdin.read(&mut buffer).unwrap();

            self.rand.write_u8(buffer[0]);

            match buffer[0] {
                b'r' => return true,
                b'q' => return false,
                _ => {}
            }
        }
    }

    fn check_game_over(&self) -> bool {
        let head = self.player.body.back().unwrap();

        self.player
            .body
            .iter()
            .filter(|part| (head.x, head.y) == (part.x, part.y))
            .count()
            > 1
            || head.x == 1
            || head.x == self.width
            || head.y == 1
            || head.y == self.height
    }

    fn update(&mut self) {
        self.player.update(&mut self.food);
        self.food.update(self.width, self.height);
    }

    fn draw(&mut self) {
        write!(self.stdout, "{}", clear::All).unwrap();

        // Draw player
        for (i, part) in self.player.body.iter().enumerate() {
            if i % 2 == 0 {
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto(part.x, part.y),
                    color::Fg(color::Cyan)
                )
                .unwrap();
            } else {
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto(part.x, part.y),
                    color::Fg(color::Blue)
                )
                .unwrap();
            }
            self.stdout.write(b"\xE2\x96\x88").unwrap();
        }

        // Draw food
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.food.x, self.food.y),
            color::Fg(color::Red)
        )
        .unwrap();
        self.stdout.write(b"\xE2\x96\x88").unwrap();

        // Draw walls
        write!(self.stdout, "{}", color::Fg(color::Reset)).unwrap();

        const HORIZONTAL_WALL: &str = "═";
        const VERTICAL_WALL: &str = "║";
        const CORNERS: (&str, &str, &str, &str) = ("╔", "╗", "╝", "╚");

        // Horizontal wall
        for i in 1..=self.width {
            write!(self.stdout, "{}{}", cursor::Goto(i, 1), HORIZONTAL_WALL).unwrap();
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(i, self.height),
                HORIZONTAL_WALL
            )
            .unwrap();
        }

        // Vertical wall
        for i in 1..=self.height {
            write!(self.stdout, "{}{}", cursor::Goto(1, i), VERTICAL_WALL).unwrap();
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(self.width, i),
                VERTICAL_WALL
            )
            .unwrap();
        }

        write!(self.stdout, "{}{}", cursor::Goto(1, 1), CORNERS.0).unwrap();
        write!(self.stdout, "{}{}", cursor::Goto(self.width, 1), CORNERS.1).unwrap();
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.width, self.height),
            CORNERS.2
        )
        .unwrap();
        write!(self.stdout, "{}{}", cursor::Goto(1, self.height), CORNERS.3).unwrap();

        // Draw UI
        let score = self.player.body.len();
        let score_text: String = format!("SCORE: {score}");

        write!(
            self.stdout,
            "{}{}",
            cursor::Goto((self.width - score_text.len() as u16) / 2, 1),
            score_text
        )
        .unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.stdout, "{}{}", clear::All, style::Reset).unwrap();

        self.player = Player::new(self.width, self.height);
        self.food = Food::new(self.width, self.height);

        self.score = 0;
        self.speed = 10;
    }
}
