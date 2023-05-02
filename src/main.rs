pub mod model;
pub mod utils;

use model::{Space, Cube};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

fn model(app: &App) -> Space {
    let window_id = app
        .new_window()
        .view(view)
        .title("Universim")
        .build()
        .unwrap();

    let cubes = vec![Cube::new(0., 0., 0., 500.)];
    
    Space::new(window_id, cubes)
}

fn update(app: &App, space: &mut Space, update: Update) {
    space.update(app, update)
}

fn event(app: &App, space: &mut Space, event: Event) {
    space.event(app, event)
}

fn view(app: &App, space: &Space, frame: Frame) {
    space.draw(app, frame);
}
