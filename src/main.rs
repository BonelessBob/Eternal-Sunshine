use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    win_id: WindowId,
    texture: wgpu::Texture,
    bg_color: Srgb<u8>,
    x: f32,
    y: f32,
    radius: f32,
}

fn model(app: &App) -> Model {
    let win_id = app.new_window()
        .size(500, 500)
        .view(view)
        .build().unwrap();
    // app.window(win_id).unwrap().set_fullscreen(true);


    let texture = wgpu::Texture::from_path(
        app,
        "./assets/kyst.jpg"
    ).unwrap();


    let texture_dims = texture.extent();
    app.window(win_id).unwrap().set_max_inner_size_points(Some((texture_dims.width as f32, texture_dims.height as f32)));
    app.window(win_id).unwrap().set_maximized(true);
    // Do I make sure that texture_dims.width returns pixels?



    Model {
        win_id,
        texture,
        bg_color: HONEYDEW,
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _model.radius < _app.window_rect().pad(25.0).w() / 2.0 {
        _model.radius += 1.0;
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.background().color(_model.bg_color);
    draw.texture(&_model.texture);

    draw.ellipse()
        .color(DARKSLATEGRAY)
        .w_h(_model.radius, _model.radius)
        .x_y(_model.x, _model.y);

    draw.to_frame(app, &frame).unwrap();
}
