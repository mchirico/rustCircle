use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Generate sine wave data based on the time of the app
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    // Get boundary of the window (to constrain the movements of our circle)
    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse at the x/y coordinates 0.0, 0.0
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
