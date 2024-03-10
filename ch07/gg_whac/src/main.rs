use std::{rc::Rc, time::Duration};

use ggez::{
    audio::{self, SoundSource},
    conf,
    event::{self, EventHandler, MouseButton},
    graphics::{self, Canvas, DrawParam, Drawable, FontData, PxScale, Rect, TextFragment},
    input::mouse::{self, set_cursor_type},
    mint::{Point2, Vector2},
    timer, Context, ContextBuilder, GameResult,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

const N_COLUMNS: usize = 5;
const N_ROWS: usize = 3;
const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;
const FIRST_COLUMN_X: f32 = 60.;
const COLUMN_STEP: f32 = 140.;
const FIRST_ROW_Y: f32 = 140.;
const ROWS_STEP: f32 = 150.;
const GAME_DURATION: Duration = Duration::from_secs(40);
const WIDGET_TOP_MARGIN: f32 = 8.;
const WIDGET_BOTTOM_MARGIN: f32 = 8.;
const WIDGET_LEFT_MARGIN: f32 = 10.;
const WIDGET_RIGHT_MARGIN: f32 = 10.;
const BUTTON_FONT_SIZE: f32 = 44.;
const BUTTON_PRESS_SHIFT: f32 = 4.;
const DESIRED_FPS: u32 = 20;
const MALLET_SCALE: f32 = 0.3;
const MOLE_SCALE: f32 = 0.3;

#[derive(Debug)]
enum Mode {
    Ready,
    Raising,
    Lowering,
}

struct Screen {
    mode: Mode,
    start_time: Option<Duration>,
    active_mole_column: usize,
    active_mole_row: usize,
    active_mole_position: f32,
    n_hit_moles: u32,
    random_generator: ThreadRng,
    mallet_image: graphics::Image,
    lawn_image: graphics::Image,
    mole_image: graphics::Image,
    font: String,
    appearance_sound: audio::Source,
    hit_sound: audio::Source,
    miss_sound: audio::Source,
    finish_sound: audio::Source,
    mouse_down_at: Option<Point2<f32>>,
    mouse_up_at: Option<Point2<f32>>,
    start_button: Button,
}

impl Screen {
    fn new(ctx: &mut Context) -> GameResult<Screen> {
        let button_image = Rc::new(graphics::Image::from_path(ctx, "/button.png")?);
        let font_name = "LiberationMono-Regular".to_string();
        let s = Screen {
            mode: Mode::Ready,
            start_time: None,
            active_mole_column: 0,
            active_mole_row: 0,
            active_mole_position: 0.,
            n_hit_moles: 0,
            random_generator: thread_rng(),
            mallet_image: graphics::Image::from_path(ctx, "/mallet.png")?,
            lawn_image: graphics::Image::from_path(ctx, "/lawn.jpg")?,
            mole_image: graphics::Image::from_path(ctx, "/mole.png")?,
            font: font_name.clone(),
            appearance_sound: audio::Source::new(ctx, "/cry.ogg")?,
            hit_sound: audio::Source::new(ctx, "/click.ogg")?,
            miss_sound: audio::Source::new(ctx, "/bump.ogg")?,
            finish_sound: audio::Source::new(ctx, "/two_notes.ogg")?,
            mouse_down_at: None,
            mouse_up_at: None,
            start_button: Button::new(
                ctx,
                "Start",
                Point2 { x: 600., y: 40. },
                font_name.clone(),
                button_image.clone(),
            ),
        };
        ctx.gfx
            .add_font(&font_name.clone(), FontData::from_path(ctx, "/font.ttf")?);
        Ok(s)
    }

    fn get_active_mole_bounding_box(&self) -> Rect {
        Rect::new(
            FIRST_COLUMN_X + self.active_mole_column as f32 * COLUMN_STEP,
            FIRST_ROW_Y + self.active_mole_row as f32 * ROWS_STEP
                - MOLE_SCALE * self.active_mole_position * self.mole_image.height() as f32,
            MOLE_SCALE * self.mole_image.height() as f32,
            MOLE_SCALE * self.active_mole_position * self.mole_image.height() as f32,
        )
    }

    fn raise_another_mole(&mut self, ctx: &mut Context) {
        loop {
            let new_active_mole_column = self.random_generator.gen_range(0..N_COLUMNS);
            let new_active_mole_row = self.random_generator.gen_range(0..N_ROWS);
            if new_active_mole_column != self.active_mole_column
                || new_active_mole_row != self.active_mole_row
            {
                self.active_mole_column = new_active_mole_column;
                self.active_mole_row = new_active_mole_row;
                break;
            }
        }
        self.active_mole_position = 0.;
        self.mode = Mode::Raising;
        self.appearance_sound.play(ctx).unwrap();
    }
}

impl EventHandler for Screen {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.mode {
                Mode::Ready => {
                    if let Some(mouse_down_at) = self.mouse_down_at {
                        if let Some(mouse_up_at) = self.mouse_up_at {
                            if self.start_button.contains(mouse_down_at)
                                && self.start_button.contains(mouse_up_at)
                            {
                                self.mouse_down_at = None;
                                self.mouse_up_at = None;
                                self.start_time = Some(timer::time_since_start(ctx));
                                self.n_hit_moles = 0;
                                self.raise_another_mole(ctx);
                            }
                        }
                    }
                }
                Mode::Raising => {
                    if timer::time_since_start(ctx) - self.start_time.unwrap() >= GAME_DURATION {
                        self.mode = Mode::Ready;
                        self.active_mole_position = 0.;
                        self.mouse_down_at = None;
                        self.mouse_up_at = None;
                        self.finish_sound.play(ctx).unwrap();
                    } else {
                        self.active_mole_position =
                            (self.active_mole_position + 2.4 / DESIRED_FPS as f32).min(1.);
                        if let Some(mouse_pos) = self.mouse_down_at {
                            self.mouse_down_at = None;
                            if self.get_active_mole_bounding_box().contains(mouse_pos) {
                                self.mode = Mode::Lowering;
                                self.n_hit_moles += 1;
                                self.hit_sound.play(ctx).unwrap();
                            } else {
                                self.miss_sound.play(ctx).unwrap();
                            }
                        }
                    }
                }
                Mode::Lowering => {
                    self.mouse_down_at = None;
                    self.mouse_up_at = None;
                    self.active_mole_position -= 3.6 / DESIRED_FPS as f32;
                    if self.active_mole_position <= 0. {
                        self.raise_another_mole(ctx);
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let area = ctx.gfx.drawable_size();

        let lawn_params = DrawParam::new().scale(Vector2 {
            x: area.0 / self.lawn_image.width() as f32,
            y: area.1 / self.lawn_image.height() as f32,
        });

        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        canvas.draw(&self.lawn_image, lawn_params);

        if let Mode::Ready = self.mode {
            self.start_button.draw(ctx, &mut canvas).unwrap();
        }

        let bounding_box = self.get_active_mole_bounding_box();
        canvas.draw(
            &self.mole_image,
            DrawParam::new()
                .src(Rect {
                    x: 0.,
                    y: 0.,
                    w: 1.,
                    h: self.active_mole_position,
                })
                .dest(Point2 {
                    x: bounding_box.left(),
                    y: bounding_box.top(),
                })
                .scale(Vector2 {
                    x: MOLE_SCALE,
                    y: MOLE_SCALE,
                }),
        );

        let mouse_pos = ctx.mouse.position();
        if let Mode::Ready = self.mode {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::Default);
        } else if bounding_box.contains(mouse_pos) {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::Crosshair);
            let angle_degrees = match self.mode {
                Mode::Lowering => 135. - 55. * self.active_mole_position,
                _ => 80.,
            };
            canvas.draw(
                &self.mallet_image,
                DrawParam::new()
                    .dest(Point2 {
                        x: mouse_pos.x + self.mallet_image.width() as f32 * MALLET_SCALE,
                        y: mouse_pos.y,
                    })
                    .scale(Vector2 {
                        x: MALLET_SCALE,
                        y: MALLET_SCALE,
                    })
                    .offset(Point2 { x: 0., y: 1. })
                    .rotation(angle_degrees / -180. * std::f32::consts::PI),
            )
        } else {
            mouse::set_cursor_type(ctx, mouse::CursorIcon::NotAllowed);
        }

        let time_text = if let Some(start_time) = self.start_time {
            let elapsed_time = timer::time_since_start(ctx) - start_time;
            if elapsed_time < GAME_DURATION {
                format!(
                    "Remaining time: {} seconds",
                    (GAME_DURATION - elapsed_time).as_secs()
                )
            } else {
                "Game finished. Click on Start to play again.".to_string()
            }
        } else {
            "Click on Start to play.".to_string()
        };
        let text = format!(
            "{}\n\
                                    Hit moles: {}",
            time_text, self.n_hit_moles
        );
        let drawable_text = graphics::Text::new(graphics::TextFragment {
            text,
            font: Some(self.font.clone()),
            scale: Some(PxScale::from(24.)),
            ..Default::default()
        });
        canvas.draw(
            &drawable_text,
            DrawParam::new()
                .dest(Point2 { x: 4., y: 4. })
                .color(graphics::Color::BLACK),
        );
        canvas.draw(
            &drawable_text,
            DrawParam::new()
                .dest(Point2 { x: 2., y: 2. })
                .color(graphics::Color::WHITE),
        );

        canvas.finish(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.mouse_down_at = Some(Point2 { x, y });
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            self.mouse_up_at = Some(Point2 { x, y });
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Button {
    base_image: Rc<graphics::Image>,
    bounding_box: Rect,
    drawable_text: graphics::Text,
}

impl Button {
    fn new(
        ctx: &mut Context,
        caption: &str,
        center: Point2<f32>,
        font: String,
        base_image: Rc<graphics::Image>,
    ) -> Self {
        let drawable_text = graphics::Text::new(TextFragment {
            text: caption.to_string(),
            font: Some(font),
            scale: Some(PxScale::from(BUTTON_FONT_SIZE)),
            ..Default::default()
        });
        let rect = drawable_text.dimensions(ctx).unwrap();
        let bounding_box = Rect::new(
            center.x - rect.w as f32 * 0.5 - WIDGET_LEFT_MARGIN,
            center.y - rect.h as f32 * 0.5 - WIDGET_TOP_MARGIN,
            rect.w as f32 + WIDGET_LEFT_MARGIN + WIDGET_RIGHT_MARGIN,
            rect.h as f32 + WIDGET_TOP_MARGIN + WIDGET_BOTTOM_MARGIN,
        );
        Button {
            base_image,
            bounding_box,
            drawable_text,
        }
    }

    fn contains(&self, pt: Point2<f32>) -> bool {
        self.bounding_box.contains(pt)
    }

    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let mut rect = self.bounding_box;
        let is_pressed = ctx.mouse.button_pressed(MouseButton::Left);
        let is_inside = rect.contains(ctx.mouse.position());

        if is_pressed && is_inside {
            rect.y += BUTTON_PRESS_SHIFT;
        }
        let mut area_draw_param = DrawParam::new().dest(rect.point()).scale(Vector2 {
            x: rect.w / (self.base_image.width() as f32),
            y: rect.h / (self.base_image.height() as f32),
        });
        if is_pressed && is_inside {
            area_draw_param = area_draw_param.src(Rect {
                x: 0.,
                y: 0.,
                w: 1.,
                h: 1. - BUTTON_PRESS_SHIFT / rect.h,
            });
        }
        canvas.draw(&*self.base_image, area_draw_param);
        canvas.draw(
            &self.drawable_text,
            DrawParam::new()
                .dest(Point2 {
                    x: rect.left() + WIDGET_LEFT_MARGIN,
                    y: rect.top() + WIDGET_TOP_MARGIN,
                })
                .color(if is_inside {
                    [0.8, 0., 0., 1.].into()
                } else {
                    graphics::Color::BLACK
                }),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    let (mut context, animation_loop) = ContextBuilder::new("whac-a-mole", "ggez")
        .window_setup(conf::WindowSetup::default().title("Whac-a-Mole"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .add_resource_path("assets")
        .build()?;
    let game = Screen::new(&mut context)?;
    event::run(context, animation_loop, game)
}
