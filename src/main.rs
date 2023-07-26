use raylib::prelude::*;
use raylib::core::window;

struct Ball {
    x: f64,
    y: f64,

    max_x: f64,
    max_y: f64,

    x_speed: f64,
    y_speed: f64,

    rl: RaylibHandle
}

impl Ball {
    fn new(w: i32, h: i32, start_x: f64, start_y: f64, start_x_speed: f64, start_y_speed: f64) -> Self {
        let (rl, _thread) = raylib::init()
            .size(w, h)
            .title("Ball")
            .build();

        Self {
            x: start_x,
            y: start_y,
            
            max_x: (get_monitor_width(0) - w) as f64,
            max_y: (get_monitor_height(0) - h) as f64,

            x_speed: start_x_speed,
            y_speed: start_y_speed,
            rl
        }
    }

    fn tick(&mut self) {
        if self.x < 0.0 || self.x > self.max_x {
            self.x_speed *= -1.0;
        }
        if self.y < 0.0 || self.y > self.max_y {
            self.y_speed *= -1.0;
        }

        self.x += self.x_speed;
        self.y += self.y_speed;

        self.rl.set_window_position(self.x as i32, self.y as i32);
    }
}

fn main() {
    let mut ball = Ball::new(300, 300, 230.0, 540.0, -0.3, 0.4);

    loop {
        ball.tick();
    }
}
