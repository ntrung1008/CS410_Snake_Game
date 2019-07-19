extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Game 
{
    gl: GlGraphics,
	snake: Snake, 
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
    }
}
struct Snake {
	x :i32,
	y :i32,
}
impl Snake{
	fn render(&self,gl: &mut GlGraphics, args:&RenderArgs)
	{
		use graphics;
		const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square(self.x as f64, self.y as f64, 20_f64);
		gl.draw(args.viewport(),|c,gl|
		{
			let transform = c.transform;
			graphics::rectangle(RED,square,transform,gl);
		})
	}
}
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
	let mut game= Game {
		gl:GlGraphics::new(opengl),
		snake: Snake{x:50,y:100},
	};	
	
	let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
	}
}