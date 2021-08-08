use nannou::prelude::*;

const DIAMETER: f32 = 64.0;

struct Model {
    pub x: f32,
    pub y: f32,
    // Navigation
    pub navigating_to: Option<(f32, f32)>,
    // Environment
    pub red_circle: (f32, f32),
    pub red_circle_interacted: bool,
    pub green_circle: (f32, f32),
    pub green_circle_interacted: bool,
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    Model {
        x: 0.0,
        y: 0.0,
        navigating_to: None,
        red_circle: (256.0, 0.0),
        red_circle_interacted: false,
        green_circle: (0.0, 256.0),
        green_circle_interacted: false,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {}

fn event(app: &App, model: &mut Model, event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // draw character
    draw.ellipse()
        .color(STEELBLUE)
        .w(DIAMETER)
        .h(DIAMETER)
        .x_y(model.x, model.y);

    // draw goals
    if model.red_circle_interacted == false {
        draw.ellipse()
            .color(ORANGERED)
            .w(DIAMETER)
            .h(DIAMETER)
            .x_y(model.red_circle.0, model.red_circle.1);
    }
    if model.green_circle_interacted == false {
        draw.ellipse()
            .color(GREENYELLOW)
            .w(DIAMETER)
            .h(DIAMETER)
            .x_y(model.green_circle.0, model.green_circle.1);
    }

    // set background to blue
    draw.background().color(PINK);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
