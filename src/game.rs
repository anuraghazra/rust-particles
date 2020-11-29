extern crate piston;
extern crate piston_window;
use piston_window::{
  clear, EventLoop, MouseCursorEvent, PistonWindow, RenderEvent, WindowSettings,
};

#[path = "particle.rs"]
mod particle;
use particle::Particle;

pub struct Game {
  width: u32,
  height: u32,
  window: PistonWindow,
  cursor: [f64; 2],
  particles: Vec<Particle>,
}

impl Game {
  pub fn new(width: u32, height: u32) -> Game {
    let mut window: PistonWindow = WindowSettings::new("Particle System", [width, height])
      .exit_on_esc(true)
      .build()
      .unwrap();

    window.set_max_fps(60);

    let cursor = [0.0, 0.0];

    let particles = Vec::new();
    return Game {
      width,
      height,
      window,
      cursor,
      particles,
    };
  }

  // https://www.reddit.com/r/rust/comments/340c3a/get_the_borrowck_error_when_borrow_self_twice_in/
  pub fn main_loop(&mut self) {
    let _self = &mut self.window;
    let particles = &mut self.particles;
    let width = self.width;
    let height = self.height;
    let mut c = [0.0, 0.0];
    while let Some(e) = _self.next() {
      e.mouse_cursor(|pos| c = pos);
      self.cursor = c;
      particles.push(Particle::new(self.cursor[0], self.cursor[1], 1.0));
      if let Some(_) = e.render_args() {
        // do calculations on next render phase
        _self.draw_2d(&e, |context, graphics, _device| {
          clear([0.0, 0.0, 0.0, 1.0], graphics);
          particles.retain(|particle| {
            let delete = { particle.life <= 0.0 };
            !delete
          });
          for x in 0..particles.len() {
            let p = particles.get_mut(x).unwrap();
            p.update(width, height);
            p.draw(&context, graphics);
          }
        });
      }
    }
  }
}
