use turtle::{
    Turtle,
    Point,
    Angle,
};
use api::RenderConfig;
use crate::Renderer;

struct State {
    position: Point,
    heading: Angle,
}

impl State {
    fn new(position: Point, heading: Angle) -> Self {
        Self { position, heading }
    }
}


pub struct Crab {
    step: f64,
    angle: f64,
    stack: Vec<State>,
    turtle: Turtle,
}

impl Crab {
    pub fn new(config: RenderConfig) -> Self {
        let mut turtle = Turtle::new();
        // colors can be any of: https://docs.rs/turtle/1.0.0-rc.2/turtle/color/static.COLOR_NAMES.html
        turtle.drawing_mut().set_background_color("white");
        turtle.set_pen_color("black");
        turtle.use_degrees();
        turtle.set_heading(0f64);

        Self {
            step: config.step as f64,
            angle: config.angle as f64,
            stack: Vec::new(),
            turtle,
        }
    }
}

impl Renderer for Crab {

    fn global_init() {
        turtle::start();
    }

    fn forward(&mut self) {
        self.turtle.forward(self.step);
    }

    fn rotate_left(&mut self) {
        self.turtle.left(self.angle);
    }

    fn rotate_right(&mut self) {
        self.turtle.right(self.angle);
    }
    fn push(&mut self) {
        let position = self.turtle.position();
        let heading = self.turtle.heading();
        let state = State::new(position, heading);
        self.stack.push(state);
    }

    fn pop(&mut self) {
        if let Some(state) = self.stack.pop() {
            self.turtle.pen_up();
            self.turtle.go_to(state.position);
            self.turtle.set_heading(state.heading);
            self.turtle.pen_down();
        }
    }

    fn finish(&mut self) {
        self.turtle.hide();
    }

}


