extern crate piston_window;
use piston_window::{rectangle, Context, G2d};

use rand::Rng;

pub struct Particle {
  x: f64,
  y: f64,
  vx: f64,
  vy: f64,
  radius: f64,
  color: [f32; 3],
  gravity: f64,
  pub life: f32,
}

impl Particle {
  pub fn new(x: f64, y: f64, gravity: f64) -> Particle {
    let mut rng = rand::thread_rng();
    let color: [f32; 3] = [
      rng.gen_range(0.1, 1.0),
      rng.gen_range(0.1, 1.0),
      rng.gen_range(0.1, 1.0),
    ];
    return Particle {
      x,
      y,
      vx: rng.gen_range(-5.0, 5.0),
      vy: rng.gen_range(-5.0, 5.0),
      radius: 4.0,
      color,
      gravity,
      life: 1.0,
    };
  }

  pub fn update(&mut self, width: u32, height: u32) {
    self.x += self.vx;
    self.y += self.vy;
    self.vy += self.gravity;

    if self.y - self.radius > height.into() {
      self.vy *= -0.8;
      self.y = height.into();
    }
    if self.y < self.radius {
      self.vy *= 0.8;
      self.y = 0.0;
    }
    if self.x - self.radius > width.into() {
      self.vx *= -0.8;
      self.x = width.into();
    }
    if self.x < 0.0 {
      self.vx *= 0.8;
      self.x = 1.0;
    }
    self.life -= 0.05;
  }

  pub fn draw(&mut self, con: &Context, g: &mut G2d) {
    rectangle(
      [self.color[0], self.color[1], self.color[2], self.life],
      [self.x, self.y, 5.0, 5.0],
      con.transform,
      g,
    );
  }
}
