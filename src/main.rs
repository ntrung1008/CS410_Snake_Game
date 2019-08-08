extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::Transformed;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::Rng;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
enum Screen {
	Menu,
	Game,
	Ready,
	GameOver,
}

#[derive(Clone, PartialEq)]
enum MenuOption {
	OnePlayer,
	TwoPlayer,
	Options,
}

// MAKE SURE WINDOWSIZE IS A MULTIPLE OF BOXSIZE
static WINDOWSIZE: (u32, u32) = (800, 800);
static RESPAWN_ENEMY: u32 = 50;
static BOXSIZE: u32 = 20;
static UPS: u64 = 10;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    snake2: Snake,
    food: Food,
    enemy: Enemy,
    ate_food: bool,
    score: u32,
    score2: u32,
    screen: Screen,
    hover: MenuOption,
}

impl Game {
    fn render(&mut self, glyphs: &mut GlyphCache<'static>, args: &RenderArgs) {
        if self.screen == Screen::Menu {
            self.render_menu(glyphs, args);
        }
        if self.screen == Screen::Game {
            if self.hover == MenuOption::OnePlayer {
                self.snake2.alive = false;
            }
            self.render_game(glyphs, args);
        }
    }

    fn render_menu(&mut self, glyphs: &mut GlyphCache<'static>, args: &RenderArgs) {
        let hover_offset = match self.hover {
			MenuOption::OnePlayer => 3,
			MenuOption::TwoPlayer => 4,
			MenuOption::Options   => 5,
		};
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(graphics::color::WHITE, gl);

            // Draw the title
            let mut trans = c
                .transform
                .trans((WINDOWSIZE.0 / 4) as f64, (WINDOWSIZE.1 / 3) as f64);
            graphics::text::Text::new(WINDOWSIZE.0 / 4)
                .draw("SNEK", glyphs, &c.draw_state, trans, gl)
                .unwrap();

            // Draw menu options
            trans = c.transform.trans(
                (WINDOWSIZE.0 as f64 * 0.3) as f64,
                (WINDOWSIZE.1 / 6 * 3) as f64,
            );
            graphics::text::Text::new(WINDOWSIZE.0 / 14)
                .draw("1 Player", glyphs, &c.draw_state, trans, gl)
                .unwrap();
            trans = c.transform.trans(
                (WINDOWSIZE.0 as f64 * 0.3) as f64,
                (WINDOWSIZE.1 / 6 * 4) as f64,
            );
            graphics::text::Text::new(WINDOWSIZE.0 / 14)
                .draw("2 Player", glyphs, &c.draw_state, trans, gl)
                .unwrap();
            trans = c.transform.trans(
                (WINDOWSIZE.0 as f64 * 0.3) as f64,
                (WINDOWSIZE.1 / 6 * 5) as f64,
            );
            graphics::text::Text::new(WINDOWSIZE.0 / 14)
                .draw("options", glyphs, &c.draw_state, trans, gl)
                .unwrap();
            trans = c.transform.trans(0 as f64, (WINDOWSIZE.1 - 5) as f64);
            graphics::text::Text::new(WINDOWSIZE.0 / 30)
                .draw("press ESC to quit...", glyphs, &c.draw_state, trans, gl)
                .unwrap();

            // Draw circle selector
            trans = c.transform;
            let ellip = graphics::rectangle::square(
                WINDOWSIZE.0 as f64 * 0.3 - 20.0,
                (WINDOWSIZE.1 / 6 * (hover_offset) - 20) as f64,
                (BOXSIZE / 2) as f64,
            );
            graphics::ellipse(graphics::color::BLACK, ellip, trans, gl);
        });
    }

    fn render_game(&mut self, glyphs: &mut GlyphCache<'static>, args: &RenderArgs) {
		let player_num = self.hover.clone();
        let score_str = self.score.to_string().into_boxed_str();
		let score_str2 = self.score2.to_string().into_boxed_str();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(graphics::color::WHITE, gl);

		   	// Draw P1 score
			let trans = c.transform.trans(
                (WINDOWSIZE.0 - WINDOWSIZE.0) as f64,
                (WINDOWSIZE.1 - 50) as f64,
            );
            graphics::text::Text::new(20)
                .draw("SCORE", glyphs, &c.draw_state, trans, gl)
                .unwrap();

            let trans2 = c.transform.trans(
                (WINDOWSIZE.0 - WINDOWSIZE.0) as f64,
                (WINDOWSIZE.1 - 25) as f64,
            );
            graphics::text::Text::new(20)
                .draw(&*score_str, glyphs, &c.draw_state, trans2, gl)
                .unwrap();
        });
        
		if player_num == MenuOption::TwoPlayer {
			self.gl.draw(args.viewport(), |c, gl| {
				// Draw P2 score
            		let trans = c
            		    .transform
            		    .trans((WINDOWSIZE.0 - 60) as f64, (WINDOWSIZE.1 - 50) as f64);
            		graphics::text::Text::new(20)
            		    .draw("SCORE2", glyphs, &c.draw_state, trans, gl)
            		    .unwrap();
            		let trans2 = c
            		    .transform
            		    .trans((WINDOWSIZE.0 - 30) as f64, (WINDOWSIZE.1 - 25) as f64);
            		graphics::text::Text::new(20)
            		    .draw(&*score_str2, glyphs, &c.draw_state, trans2, gl)
            		    .unwrap();
			});
        };

        // Render the rest
        self.food.render(&mut self.gl, args);
        if self.snake.alive == true {
            self.snake.render(&mut self.gl, args, 1);
        }
        if self.snake2.alive == true {
            self.snake2.render(&mut self.gl, args, 2);
        }
        self.enemy.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        if self.screen == Screen::Menu {
            self.update_menu();
        }
        if self.screen == Screen::Game {
            self.update_game();
        }
    }

    fn update_menu(&mut self) {}

    fn update_game(&mut self) {
        //Snake 1
        if self.snake.alive == true {
            self.ate_food = self.food.got_eaten(&self.snake);
            self.snake.update(self.ate_food, &mut self.snake2);
            if self.ate_food == true {
                let mut rng = rand::thread_rng();
                self.food = Food {
                    x: rng.gen_range(0, WINDOWSIZE.0 / BOXSIZE - 1),
                    y: rng.gen_range(0, WINDOWSIZE.1 / BOXSIZE - 1),
                };
                self.score += 1;
                self.ate_food = false;
            }
            if self.enemy.kill_snake(&self.snake) == true {
                self.snake.alive = false;
            }
        }

        //Snake 2
        if self.snake2.alive == true {
            self.ate_food = self.food.got_eaten(&self.snake2);
            self.snake2.update(self.ate_food, &mut self.snake);
            if self.ate_food == true {
                let mut rng = rand::thread_rng();
                self.food = Food {
                    x: rng.gen_range(0, WINDOWSIZE.0 / BOXSIZE - 1),
                    y: rng.gen_range(0, WINDOWSIZE.1 / BOXSIZE - 1),
                };
                self.score2 += 1;
                self.ate_food = false;
            }
            if self.enemy.kill_snake(&self.snake2) == true {
                self.snake2.alive = false;
            }
        }

        self.enemy.spawn -= 1;
        if self.enemy.spawn <= 0 {
            let mut rng = rand::thread_rng();
            self.enemy = Enemy {
                x: rng.gen_range(0, WINDOWSIZE.0 / BOXSIZE - 1),
                y: rng.gen_range(0, WINDOWSIZE.1 / BOXSIZE - 1),
                spawn: RESPAWN_ENEMY,
            };
        }
    }

    fn pressed(&mut self, btn: &Button, sta: &ButtonState) {
        if self.screen == Screen::Menu {
            self.pressed_menu(btn, sta);
        }
        if self.screen == Screen::Game {
            self.pressed_game(btn);
        }
    }

    fn pressed_menu(&mut self, btn: &Button, sta: &ButtonState) {
        match btn {
            &Button::Keyboard(Key::Up) => {
                if sta == &ButtonState::Press {
                    self.hover = match self.hover {
                        MenuOption::TwoPlayer => MenuOption::OnePlayer,
						MenuOption::Options   => MenuOption::TwoPlayer,
						_					  => MenuOption::Options,
                    }
                }
            }
            &Button::Keyboard(Key::Down) => {
                if sta == &ButtonState::Press {
                    self.hover = match self.hover {
                        MenuOption::OnePlayer => MenuOption::TwoPlayer,
						MenuOption::TwoPlayer => MenuOption::Options,
						_					  => MenuOption::OnePlayer,
                    }
                }
            }
            &Button::Keyboard(Key::Return) => self.screen = Screen::Game,
            _ => self.screen = Screen::Menu,
        }
    }

    fn pressed_game(&mut self, btn: &Button) {
		if self.hover == MenuOption::TwoPlayer {
        	let current_direction2 = self.snake2.dir.clone();
        	self.snake2.dir = match btn {
        	    &Button::Keyboard(Key::Up) if current_direction2 != Direction::Down => Direction::Up, //if the snake is not going down then change it to go up
        	    &Button::Keyboard(Key::Down) if current_direction2 != Direction::Up => Direction::Down,
        	    &Button::Keyboard(Key::Left) if current_direction2 != Direction::Right => {
        	        Direction::Left
        	    }
        	    &Button::Keyboard(Key::Right) if current_direction2 != Direction::Left => {
        	        Direction::Right
        	    }
        	    _ => current_direction2,
        	};
		}

        let current_direction = self.snake.dir.clone();
        self.snake.dir = match btn {
            &Button::Keyboard(Key::W) if current_direction != Direction::Down => Direction::Up, //if the snake is not going down then change it to go up
            &Button::Keyboard(Key::S) if current_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::A) if current_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::D) if current_direction != Direction::Left => Direction::Right,
            _ => current_direction,
        };
    }
}

struct Food {
    x: u32,
    y: u32,
}

impl Food {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let ellipse = graphics::rectangle::square(
            (self.x * BOXSIZE) as f64,
            (self.y * BOXSIZE) as f64,
            BOXSIZE as f64,
        );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::ellipse(RED, ellipse, transform, gl);
        })
    }

    fn got_eaten(&mut self, snake: &Snake) -> bool {
        let head: (u32, u32);
        // Get snek head
        match snake.snek.last() {
            Some(s) => head = *s,
            None => panic!("snek length 0"),
        }
        if head.0 == self.x && head.1 == self.y {
            return true;
        }
        return false;
    }
}

struct Snake {
    snek: Vec<(u32, u32)>,
    dir: Direction,
    alive: bool,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, player: u8) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            for piece in &self.snek {
                let square = graphics::rectangle::square(
                    (piece.0 * BOXSIZE) as f64,
                    (piece.1 * BOXSIZE) as f64,
                    BOXSIZE as f64,
                );
                if player == 1 {
                    graphics::rectangle(graphics::color::BLACK, square, transform, gl);
                } else {
                    graphics::rectangle(RED, square, transform, gl);
                }
            }
        })
    }

    fn update(&mut self, eaten: bool, other_snake: &mut Snake) {
        let head: (u32, u32);
        // Get snek head
        match self.snek.last() {
            Some(s) => head = *s,
            None => panic!("snek length 0"),
        }
        // Check for death by wall
        let next: (u32, u32) = match self.dir {
            Direction::Left => {
                if head.0 == 0 {
                    self.alive = false;
                    (0, 0)
                } else {
                    (head.0 - 1, head.1)
                }
            }
            Direction::Right => {
                if head.0 >= WINDOWSIZE.0 / BOXSIZE - 1 {
                    self.alive = false;
                }
                (head.0 + 1, head.1)
            }
            Direction::Up => {
                if head.1 == 0 {
                    self.alive = false;
                    (0, 0)
                } else {
                    (head.0, head.1 - 1)
                }
            }
            Direction::Down => {
                if head.1 >= WINDOWSIZE.1 / BOXSIZE - 1 {
                    self.alive = false;
                }
                (head.0, head.1 + 1)
            }
        };

        //logic if run into self
        if self.snek.contains(&next) {
            self.alive = false;
        }
        self.snek.push(next);
        if !eaten {
            self.snek.remove(0);
        }

        //logic if run into other
        let other: (u32, u32);
        match other_snake.snek.last() {
            Some(s) => other = *s,
            None => panic!("other snek length 0"),
        }
        if self.snek.contains(&other) {
            other_snake.alive = false
        };
    }
}

struct Enemy {
    x: u32,
    y: u32,
    spawn: u32,
}

impl Enemy {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        let ellipse = graphics::rectangle::square(
            (self.x * BOXSIZE) as f64,
            (self.y * BOXSIZE) as f64,
            BOXSIZE as f64,
        );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::ellipse(BLACK, ellipse, transform, gl);
        })
    }

    fn kill_snake(&mut self, snake: &Snake) -> bool {
        let head: (u32, u32);
        // Get snek head
        match snake.snek.last() {
            Some(s) => head = *s,
            None => panic!("snek length 0"),
        }
        if head.0 == self.x && head.1 == self.y {
            return true;
        }
        return false;
    }
}

fn main() {
    // Check grid
    assert!(
        WINDOWSIZE.0 % BOXSIZE == 0,
        "WINDOWSIZE must be a multiple of BOXSIZE"
    );
    assert!(
        WINDOWSIZE.1 % BOXSIZE == 0,
        "WINDOWSIZE must be a multiple of BOXSIZE"
    );

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake", WINDOWSIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(false)
        .build()
        .unwrap();

    // Init font
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    // Font from Ryoichi Tsunekawa at Adobe Fonts
    let mut glyphs = GlyphCache::new("src/BebasNeue-Regular.ttf", (), texture_settings)
        .expect("could not load font");

    let mut rng = rand::thread_rng();
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            snek: vec![(0,0),(1,0),(2,0)],
            dir: Direction::Right,
            alive: true,
        },
        snake2: Snake {
            snek: vec![(WINDOWSIZE.0/BOXSIZE-1,WINDOWSIZE.1/BOXSIZE-1),
						(WINDOWSIZE.0/BOXSIZE-2,WINDOWSIZE.1/BOXSIZE-2),
						(WINDOWSIZE.0/BOXSIZE-3,WINDOWSIZE.1/BOXSIZE-3)],
            dir: Direction::Left,
            alive: true,
        },
        food: Food {
            x: rng.gen_range(0, (WINDOWSIZE.0 / BOXSIZE) - 1),
            y: rng.gen_range(0, (WINDOWSIZE.1 / BOXSIZE) - 1),
        },
        ate_food: false,
        score: 0,
        score2: 0,
        enemy: Enemy {
            x: rng.gen_range(0, (WINDOWSIZE.0 / BOXSIZE) - 1),
            y: rng.gen_range(0, (WINDOWSIZE.1 / BOXSIZE) - 1),
            spawn: RESPAWN_ENEMY,
        },
        screen: Screen::Menu,
        hover: MenuOption::OnePlayer,
    };

    let mut events = Events::new(EventSettings::new()).ups(UPS); //how often to update
    while let Some(e) = events.next(&mut window) {
        if let Some(key) = e.button_args() {
            game.pressed(&key.button, &key.state);
        }

        if let Some(_u) = e.update_args() {
            game.update();
            if !game.snake.alive && !game.snake2.alive {
                break;
            }
        }

        if let Some(r) = e.render_args() {
            game.render(&mut glyphs, &r);
        }
    }
    println!("Game over. Your score is {}", game.score);
}
