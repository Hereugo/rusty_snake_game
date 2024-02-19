pub trait GameObject {
    fn update(&mut self, ctx: &crate::ctx::Ctx);
    fn render(&self, ctx: &mut crate::ctx::Ctx);
}
