use crate::game_object::GameObject;

pub struct Apple {
    pub x: i32,
    pub y: i32,
    pub size: i32,
}

impl GameObject for Apple {
    fn update(&mut self, ctx: &crate::ctx::Ctx) {
        todo!();
    }

    fn render(&self, ctx: &mut crate::ctx::Ctx) {
        todo!();
    }
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            x: 0,
            y: 0,
            size: 1,
        }
    }
}
