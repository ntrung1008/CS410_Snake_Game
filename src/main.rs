extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


#[derive(Clone,PartialEq)]
enum Direction {
	Right,Left,Up,Down
}
pub struct Game 
{
    gl: GlGraphics,
	snake: Snake, 
	food : Food,
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
		self.snake.update();
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
	x:i32,
	y:i32,
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
}
struct Snake {
	x :i32,
	y :i32,
	dir :Direction,
}
impl Snake{
	fn render(&self,gl: &mut GlGraphics, args:&RenderArgs)
	{
		use graphics;
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square((self.x *20) as f64, (self.y*20) as f64, 20_f64);
		gl.draw(args.viewport(),|c,gl|
		{
			let transform = c.transform;
			graphics::rectangle(RED,square,transform,gl);
		})
	}
	fn update (&mut self)
	{
		match self.dir {
			Direction::Left  => self.x -=1,
			Direction::Right => self.x +=1,
			Direction::Up    => self.y -=1,
			Direction::Down  => self.y +=1,
		}
	}
}
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Snake",
            [200, 200]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
	let mut game= Game {
		gl:GlGraphics::new(opengl),
		snake: Snake{x:0,y:0, dir: Direction :: Right},
		food :Food {x :1, y:1}
	};	
	
	let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
		
		if let Some(u) = e.update_args() {
			game.update();
		}
		
		if let Some(key) = e.button_args(){
			game.pressed(&key.button);
		}
	}
}