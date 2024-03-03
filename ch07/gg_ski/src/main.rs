use ggez::{
    conf,
    event::{self, EventHandler},
    glam::*,
    graphics::{self, DrawMode, Mesh, Rect},
    mint::Point2,
    timer,
    winit::event::VirtualKeyCode,
    Context, ContextBuilder, GameResult,
};
use std::f32::consts::PI;
use std::time::Duration;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SKI_WIDTH: f32 = 10.0;
const SKI_LENGTH: f32 = 50.0;
const SKI_TIP_LEN: f32 = 20.0;
const STEERING_SPEED: f32 = 110.0 / 180.0 * PI;
const MAX_ANGLE: f32 = 75.0 / 100.0 * PI;

#[derive(Debug)]
struct InputState {
    to_turn: f32,
    started: bool,
}

struct Screen {
    ski_across_offset: f32,
    direction: f32,
    previous_frame_time: Duration,
    period_in_sec: f32,
    input: InputState,
    ski: Mesh,
}

impl Screen {
    fn new(ctx: &mut Context) -> GameResult<Screen> {
        let mb = &mut graphics::MeshBuilder::new();
        mb.rectangle(
            DrawMode::fill(),
            Rect {
                x: -SKI_WIDTH / 2.0,
                y: SKI_TIP_LEN,
                w: SKI_WIDTH,
                h: SKI_LENGTH,
            },
            [1.0, 0.0, 1.0, 1.0].into(),
        )?
        .polygon(
            DrawMode::fill(),
            &[
                Point2 {
                    x: -SKI_WIDTH / 2.0,
                    y: SKI_TIP_LEN,
                },
                Point2 {
                    x: SKI_WIDTH / 2.0,
                    y: SKI_TIP_LEN,
                },
                Point2 { x: 0.0, y: 0.0 },
            ],
            [0.5, 0.0, 1.0, 1.0].into(),
        )?;

        let s = Screen {
            ski_across_offset: 0.0,
            direction: 0.0,
            previous_frame_time: Duration::from_secs(0),
            period_in_sec: 0.0,
            input: InputState {
                to_turn: 0.0,
                started: false,
            },
            ski: graphics::Mesh::from_data(ctx, mb.build()),
        };

        Ok(s)
    }

    fn steer(&mut self, side: f32) {
        if side == 0.0 {
            return;
        }
        self.direction += STEERING_SPEED * self.period_in_sec * side;
        if self.direction > MAX_ANGLE {
            self.direction = MAX_ANGLE;
        } else if self.direction < -MAX_ANGLE {
            self.direction = -MAX_ANGLE;
        }
    }
}

impl EventHandler for Screen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 25;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let now = timer::time_since_start(ctx);
            self.period_in_sec = (now - self.previous_frame_time).as_millis() as f32 / 1000.0;
            self.previous_frame_time = now;
            self.steer(self.input.to_turn);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        canvas.draw(
            &self.ski,
            graphics::DrawParam::new()
                .dest(Point2 {
                    x: SCREEN_WIDTH / 2.0 + self.ski_across_offset,
                    y: SCREEN_HEIGHT * 15.0 / 16.0 - SKI_LENGTH / 2.0 - SKI_TIP_LEN,
                })
                .rotation(self.direction),
        );
        canvas.finish(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if let Some(code) = input.keycode {
            match code {
                VirtualKeyCode::Left => {
                    self.input.to_turn = -1.0;
                }
                VirtualKeyCode::Right => {
                    self.input.to_turn = 1.0;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        if let Some(code) = input.keycode {
            match code {
                VirtualKeyCode::Left | VirtualKeyCode::Right => {
                    self.input.to_turn = 0.0;
                }
                _ => (),
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut context, animation_loop) = ContextBuilder::new("ski", "ggez")
        .window_setup(conf::WindowSetup::default().title("Ski"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("static")
        .build()?;
    let game = Screen::new(&mut context).unwrap();
    event::run(context, animation_loop, game)
}
