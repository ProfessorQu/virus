use raylib::prelude::*;
use rand::prelude::*;

struct Ball {
    x: f64,
    y: f64,

    max_x: f64,
    max_y: f64,

    x_speed: f64,
    y_speed: f64,

    rl: RaylibHandle,
    audio: RaylibAudio,
    hit_sound: Sound
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        let size = rng.gen_range(100..500);

        let (rl, _thread) = raylib::init()
            .size(size, size)
            .title("Ball")
            .build();
    
        let audio = RaylibAudio::init_audio_device();
        let hit_sound = Sound::load_sound("src/pop.mp3").unwrap();

        let max_x = (get_monitor_width(0) - size) as f64;
        let max_y = (get_monitor_height(0) - size) as f64;

        let start_x = rng.gen_range(0.0..max_x);
        let start_y = rng.gen_range(0.0..max_y);

        let start_x_speed = rng.gen_range(-1.0..1.0);
        let start_y_speed = rng.gen_range(-1.0..1.0);

        Self {
            x: start_x,
            y: start_y,
            
            max_x,
            max_y,

            x_speed: start_x_speed,
            y_speed: start_y_speed,

            rl,
            audio,
            hit_sound
        }
    }

    fn tick(&mut self) {
        if self.x < 0.0 || self.x > self.max_x {
            self.x_speed *= -1.0;

            self.audio.play_sound(&self.hit_sound);
        }
        if self.y < 0.0 || self.y > self.max_y {
            self.y_speed *= -1.0;

            self.audio.play_sound(&self.hit_sound);
        }

        self.x += self.x_speed;
        self.y += self.y_speed;

        self.rl.set_window_position(self.x as i32, self.y as i32);
    }
}

fn main() {
    let mut ball = Ball::new();

    loop {
        ball.tick();
    }
}
