use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    _window: window::Id,
}
fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {}
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    draw.line().start(pt2(win.left(),0.0)).end(pt2(win.right(), 0.0)).stroke_weight(2.0).color(WHITE);
    draw.line().start(pt2(0.0, win.top())).end(pt2(0.0, win.bottom())).stroke_weight(2.0).color(WHITE);
    let points = (0..450).map(|i| {
        let x = i as f32;          
        let point = pt2(x/15.0+5.0, (x/15.0).sin()+5.0) * 30.0;//to decide the scale
        (point, WHITE)
      });
    draw.polyline().weight(3.0).points_colored(points);
    draw.ellipse().x_y(150.0,150.0).radius(5.0).color(RED);
    draw.to_frame(app, &frame).unwrap();
}
