extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use rand::Rng;


#[derive(Clone,PartialEq)]
enum Direction {
	Right,Left,Up,Down
}

static WINDOWSIZE : (u32,u32)=(400,400);

pub struct Game 
{
    gl: GlGraphics,
	snake: Snake, 
	food : Food,
	ate_food:bool,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_c, gl|
		{
            // Clear the screen.
            clear(GREEN, gl);  
        });
		self.snake.render(&mut self.gl, args);
		self.food.render(&mut self.gl, args);
    }
	fn update( &mut self){
		self.ate_food = self.food.got_eaten(&self.snake);
		self.snake.update(self.ate_food);
		if self.ate_food== true
		{
			let mut rng = rand::thread_rng();
			self.food= Food{x:rng.gen_range(0,20),y:rng.gen_range(0,20)};
			self.ate_food=false;
		}
	}
	fn pressed(&mut self, btn :& Button){
		let current_direction = self.snake.dir.clone();
		self.snake.dir = match btn 
		{
			&Button::Keyboard(Key::Up)
				if current_direction != Direction::Down => Direction::Up, //if the snake is not going down then change it to go up
			&Button::Keyboard(Key::Down)
				if current_direction != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Left)
				if current_direction != Direction::Right => Direction::Left,
			&Button::Keyboard(Key::Right)
				if current_direction != Direction::Left => Direction::Right,
			_ => current_direction,
		};
	}
}

struct Food {
	x:u32,
	y:u32,
}

impl Food{
	fn render(&self,gl: &mut GlGraphics, args:&RenderArgs)
	{
		use graphics;
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let ellipse = graphics::rectangle::square((self.x *20) as f64, (self.y*20) as f64, 20_f64);
		gl.draw(args.viewport(),|c,gl|
		{
			let transform = c.transform;
			graphics::ellipse(RED,ellipse,transform,gl);
		})
	}
	fn got_eaten (& mut self, snake: &Snake) ->bool
	{
        let head: (u32, u32);
        // Get snek head
        match snake.snek.last() {
            Some(s) => head = *s,
            None    => panic!("snek length 0"),
        }
		if head.0 == self.x && head.1 == self.y
		{	return true;}
		return false;
	}
}

struct Snake {
    snek  :Vec<(u32, u32)>,
	dir   :Direction,
    alive :bool,
}

impl Snake{
	fn render(&self,gl: &mut GlGraphics, args:&RenderArgs)
	{
		use graphics;
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		gl.draw(args.viewport(),|c,gl|
		{
			let transform = c.transform;
            for piece in &self.snek {
                let square = graphics::rectangle::square((piece.0*20) as f64, (piece.1*20) as f64, 20_f64);
			    graphics::rectangle(RED,square,transform,gl);
            }
		})
	}

	fn update (&mut self, eaten: bool)
	{
        let head: (u32,u32);
		let mut next: (u32,u32) = (0,0);
        // Get snek head
        match self.snek.last() {
            Some(s) => head = *s,
            None    => panic!("snek length 0"),
        }
        // Check for death by wall
		match self.dir {
			Direction::Left  => if head.0 == 0 {self.alive = false}
                                else {
									next = (head.0-1,head.1);
                                },
			Direction::Right => if head.0 >= WINDOWSIZE.0/20-1 {self.alive = false}
                                else {
									next = (head.0+1,head.1);
                                },
			Direction::Up    => if head.1 == 0 {self.alive = false}
                                else {
									next = (head.0,head.1-1);
                                },
			Direction::Down  => if head.1 >= WINDOWSIZE.1/20-1 {self.alive = false}
                                else {
									next = (head.0,head.1+1);
                                },
		}
		if self.snek.contains(&next) {self.alive = false}
        self.snek.push(next);
        if !eaten {self.snek.remove(0);}
	}
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Snake",
            WINDOWSIZE
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
	let mut rng = rand::thread_rng();
	let mut game= Game {
		gl:GlGraphics::new(opengl),
		snake: Snake{snek: vec![(5,5),(5,6),(6,6),(6,7),(7,7),(8,7),(8,8)], dir: Direction :: Right, alive: true},
		food :Food {x :rng.gen_range(0, (WINDOWSIZE.0/20)-1), y:rng.gen_range(0, (WINDOWSIZE.1/20)-1)},
		ate_food:false,
	};	
	
	let mut events = Events::new(EventSettings::new()).ups(2); //how often to update
    while let Some(e) = events.next(&mut window) {
		if let Some(key) = e.button_args(){
			game.pressed(&key.button);
		}
		
        if let Some(u) = e.update_args() {
			game.update();
            if !game.snake.alive {
                break;
		    }
        }
		
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
	}
}
