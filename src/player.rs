use crate::game_object::GameObject;

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl GameObject for Player {
    fn update(&mut self, ctx: &crate::ctx::Ctx) {
        self.x += 1;
        
        if self.x > ctx.width {
            self.x = 0;
        }
    }

    fn render(&self, ctx: &mut crate::ctx::Ctx) {
        ctx.set(self.x, self.y, 'P');
    }
}

impl Player {
    pub fn new() -> Player {
        Player { x: 5, y: 5 }
    }
}
