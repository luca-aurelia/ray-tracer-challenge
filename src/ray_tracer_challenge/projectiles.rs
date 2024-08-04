use super::{pt2, vec2, Point2, Vec2};
use crate::{canvas::Canvas, oklcha};

struct Projectile {
    position: Point2,
    velocity: Vec2,
}

impl Projectile {
    fn update(&mut self, environment: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + environment.gravity + environment.wind;
    }
}

struct Environment {
    gravity: Vec2,
    wind: Vec2,
}

pub fn simulate_projectiles(canvas: &mut Canvas) {
    let environment = Environment {
        gravity: vec2(0.0, -0.0005).wh(),
        wind: vec2(-0.0001, 0.0).wh(),
    };

    let mut projectile = Projectile {
        position: pt2(0.0, 0.01).wh(),
        velocity: vec2(0.01, 0.03).wh(),
    };

    loop {
        projectile.update(&environment);

        let position = projectile.position.flip_y();
        let x = position.x();
        let y = position.y();

        if x >= canvas.width() || y >= canvas.height() || y < 0.0 {
            break;
        }

        let color = oklcha(0.5, 1.0, 0.5, 1.0);
        canvas.set_pixel(position, color);
    }
}
