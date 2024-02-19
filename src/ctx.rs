pub struct Ctx {
    pub width: i32,
    pub height: i32,
    display: Vec<Vec<char>>,
}

impl Ctx {
    pub fn new(width: i32, height: i32) -> Ctx {
        Ctx {
            width,
            height,
            display: vec![vec!['#'; width as usize]; height as usize],
        }
    }

    pub fn render(&mut self) {
        for row in &self.display {
            for cell in row {
                print!("{}", cell);
            }
            println!(""); // New line after each row
        }

        self.clear();
    }

    pub fn set(&mut self, x: i32, y: i32, c: char) {
        if !self.in_bounds(x, y) {
            return;
        }

        self.display[y as usize][x as usize] = c;
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width)
            && (y >= 0 && y < self.height)
            && (x >= 0 && (x as usize) < self.display[0].len())
            && (y >= 0 && (y as usize) < self.display.len())
    }

    fn clear(&mut self) {
        for row in &mut self.display {
            for cell in row {
                *cell = '#';
            }
        }

        // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printedj
        print!("\x1B[2J\x1B[1;1H");
    }
}
