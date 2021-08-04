use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    Model {}
}

fn update(app: &App, model: &mut Model, update: Update) {}

fn event(app: &App, model: &mut Model, event: Event) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // draw circle
    draw.ellipse()
        .color(STEELBLUE)
        .w(64.0)
        .h(64.0)
        .x_y(200.0, -100.0);

    // set background to blue
    draw.background().color(PINK);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
