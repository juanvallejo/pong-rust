extern crate piston_window;
extern crate sdl2_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::{WindowSettings, Size};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};

use graphics::{Transformed, Context};
use piston_window::PistonWindow;
use sdl2_window::Sdl2Window;
use opengl_graphics::{GlGraphics, OpenGL};

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 0.8];
const FOREGROUND_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.8];
const GAME_DIMS: Size = Size{width: 800, height: 600};
const SCORE_LIMIT: i32 = 5;

pub struct Paddle {
    x: i32,
    y: i32,
    vel: i32,
    vel_mod: i32,
    width: i32,
    height: i32,
}

impl Paddle {
    fn new() -> Paddle {
        Paddle {
            x: 0,
            y: 0,
            vel: 0,
            vel_mod: 2,
            width: 30,
            height: 150,
        }
    }

    fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn render(&mut self, c: Context, gl: &mut GlGraphics) {
        let rect = graphics::rectangle::square(0.0, 0.0, self.height as f64);
        graphics::rectangle(FOREGROUND_COLOR, rect, c.transform.trans(self.x as f64, self.y as f64), gl);
    }
}

pub struct Ball {
    radius: i32,
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: 50,
            y: 50,
            vel_x: 2,
            vel_y: 2,
            radius: 20,
        }
    }

    fn render(&mut self, c: Context, gl: &mut GlGraphics) {
        let ball = graphics::ellipse::circle(0.0, 0.0, self.radius as f64);
        graphics::ellipse(FOREGROUND_COLOR, ball, c.transform.trans(self.x as f64, self.y as f64), gl);
    }

    fn reset(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Game {
    gl: GlGraphics,
    left_score: i32,
    right_score: i32,

    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,

    width: i32,
    height: i32,
}

impl Game {
    fn new(opengl: OpenGL, size: Size) -> Game {
        let mut left_paddle = Paddle::new();
        let left_paddle_x = -left_paddle.height + left_paddle.width;
        let left_paddle_y = (size.height as i32 / 2) - (left_paddle.height / 2);
        // adjust left paddle's x pos to "hide" area of square > width
        // adjust left paddle's y pos to middle of screen
        left_paddle.set_pos(left_paddle_x, left_paddle_y);

        let mut right_paddle = Paddle::new();
        let right_paddle_x = size.width as i32 - right_paddle.width;
        let right_paddle_y = (size.height as i32 / 2) - (right_paddle.height / 2);
        // adjust right paddle's x pos to right of screen - width
        // adjust right paddle's y pos to middle of screen
        right_paddle.set_pos(right_paddle_x, right_paddle_y);

        Game {
            gl: GlGraphics::new(opengl),
            left_score: 0,
            right_score: 0,


            left_paddle,
            right_paddle,
            ball: Ball::new(),

            width: size.width as i32,
            height: size.height as i32,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let gl = &mut self.gl;
        let ball = &mut self.ball;
        let left_paddle = &mut self.left_paddle;
        let right_paddle = &mut self.right_paddle;

        gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BACKGROUND_COLOR, gl);

            left_paddle.render(c, gl);
            right_paddle.render(c, gl);
            ball.render(c, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.ball.y += self.ball.vel_y;
        self.ball.x += self.ball.vel_x;

        self.left_paddle.y += self.left_paddle.vel * self.left_paddle.vel_mod;
        self.right_paddle.y += self.right_paddle.vel * self.right_paddle.vel_mod;

        // determine if ball collides with either paddle
        if (self.ball.x - self.ball.radius <= self.left_paddle.width
            && self.ball.y + self.ball.radius >= self.left_paddle.y && self.ball.y - self.ball.radius <= self.left_paddle.y + self.left_paddle.height)
            || (self.ball.x + self.ball.radius >= self.width - self.right_paddle.width
            && self.ball.y + self.ball.radius >= self.right_paddle.y && self.ball.y - self.ball.radius <= self.right_paddle.y + self.right_paddle.height) {
            self.ball.vel_x = -self.ball.vel_x;

            // determine if ball collides with top or bottom of paddle
            if ((self.ball.y - self.ball.radius > self.right_paddle.y + self.right_paddle.height - 10
                    || self.ball.y + self.ball.radius < self.right_paddle.y + 10)
                    && self.ball.x + self.ball.radius >= self.width - self.right_paddle.width)
                || ((self.ball.y - self.ball.radius > self.left_paddle.y + self.left_paddle.height - 10
                    || self.ball.y + self.ball.radius < self.left_paddle.y + 10)
                    && self.ball.x - self.ball.radius <= self.left_paddle.width) {
                self.ball.vel_y = -self.ball.vel_y;
            }
        }

        // reverse velocity if ball collides with ceiling or floor
        if self.ball.y + self.ball.radius > self.height || self.ball.y < self.ball.radius {
            self.ball.vel_y = -self.ball.vel_y;
        }

        // ball can score if it entirely makes it past the left or right game boundaries
        if self.ball.x + self.ball.radius < 0 || self.ball.x - self.ball.radius > self.width {
            if self.ball.x + self.ball.radius < 0 {
                self.right_score += 1;
                println!("Right scores! {} - {}", self.left_score, self.right_score);
            } else {
                self.left_score += 1;
                println!("Left scores! {} - {}", self.left_score, self.right_score)
            }

            self.ball.reset(self.width / 2, self.height / 2);
        }


        if self.left_score >= SCORE_LIMIT || self.right_score >= SCORE_LIMIT {
            if self.left_score >= SCORE_LIMIT {
                println!("Left wins! {} - {}", self.left_score, self.right_score);
            } else {
                println!("Right wins! {} - {}", self.left_score, self.right_score);
            }

            std::process::exit(0);
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_paddle.vel = -1;
                }
                Key::Down => {
                    self.right_paddle.vel = 1;
                }
                Key::W => {
                    self.left_paddle.vel = -1;
                }
                Key::S => {
                    self.left_paddle.vel = 1;
                }
                _ => {}
            }
        }
    }

    fn release(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.right_paddle.vel = 0;
                }
                Key::Down => {
                    self.right_paddle.vel = 0;
                }
                Key::W => {
                    self.left_paddle.vel = 0;
                }
                Key::S => {
                    self.left_paddle.vel = 0;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("Pong", GAME_DIMS)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(opengl, GAME_DIMS);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }

        if let Some(b) = e.press_args() {
            game.press(&b);
        }

        if let Some(b) = e.release_args() {
            game.release(&b);
        }
    }
}
