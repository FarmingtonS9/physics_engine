//External libraries
use libm::{self};


//Directory setup
mod quantities;
pub use crate::quantities::*;

fn main() {
    println!("Hello, world!");
    println!("I am going to make a physics engine.");
    println!("First things first, what physics method are focussing on?");
    println!("Hamiltonian method would provide the most broad approach to real-world phenomena.");
    println!("However, learning Lagrangian and Newtonian physics first would likely yield more results in the near term")
}

//Environment
struct Canvas {
    canvas_width: f32,
    canvas_height: f32,
    canvas_scale: f32,
    sim_width: f32,
    sim_height: f32,
}

impl Canvas {
    fn new(canvas_width: f32, canvas_height: f32, min_sim_width: f32) -> Canvas {
        let canvas_scale = libm::fminf(canvas_width, canvas_height) / min_sim_width;
        let sim_width = canvas_width / canvas_scale;
        let sim_height = canvas_height / canvas_scale;

        Canvas {
            canvas_width,
            canvas_height,
            canvas_scale,
            sim_width,
            sim_height
        }
    }

    fn default() -> Canvas {
        let canvas = Canvas::new(1920., 1080., 20.);
        canvas
    }

    fn get_canvas_width(&self) -> f32 {
        self.canvas_width
    }

    fn get_canvas_height(&self) -> f32 {
        self.canvas_height
    }
    fn get_sim_width(&self) -> f32 {
        self.sim_width
    }
    fn get_sim_height(&self) -> f32 {
        self.sim_height
    }
}

struct Ball {
    radius: f32,
    pos: (f32, f32), //(x, y) coordinates
}

impl Ball {
    fn new(radius: f32, pos: (f32, f32)) -> Ball {
        Ball {
            radius,
            pos,
        }
    }

    fn default() -> Ball {
        let ball = Ball::new(300., (0., 0.));
        ball
    }
}



fn draw() {
    //Clear scene



}

fn simulation() {

}

fn update() {

}