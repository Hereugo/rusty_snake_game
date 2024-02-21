use rand::{rngs::ThreadRng, Rng};

pub struct Food {
    pub x: u16,
    pub y: u16,
    pub is_eaten: bool,
    rand: ThreadRng,
}

impl Food {
    pub fn new(width: u16, height: u16) -> Food {
        Food {
            x: width * 3 / 4,
            y: height / 2,
            is_eaten: false,
            rand: rand::thread_rng(),
        }
    }

    pub fn update(&mut self, width: u16, height: u16) {
        if !self.is_eaten {
            return;
        }

        self.x = self.rand.gen_range(2..width);
        self.y = self.rand.gen_range(2..height);
        self.is_eaten = false;
    }
}
