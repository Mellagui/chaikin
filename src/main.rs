use macroquad::prelude::*;

mod chaikin;
use chaikin::*;

#[macroquad::main("CHAIKIN => Left Click -> Add Point | Enter -> Animate | Esc -> Quit | c -> restar")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut tmp_points: Vec<(f32, f32)> = Vec::new();
    let mut started = false;
    let mut message = false;
    let mut step = 0;
    let mut timer = get_time();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::C) {
            started = false;
            points = Vec::new();
            step = 0;
            message = false;
        }
        if is_key_pressed(KeyCode::Enter)  {
            if points.is_empty() {
                message = true;
            } else if points.len() > 1{
                started = true;
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) && !started {
            let (x, y) = mouse_position();
            points.push(( x, y ));
            message = false;    
        }
        for point in points.iter() {
            draw_circle_lines(point.0, point.1, 2.5, 1. , RED);
        }

        if message {
            // draw_text("click on the mouse to draw the point!!!", 20.0, 20.0, 30.0, WHITE);
            let text = "click on the mouse to draw the point!!!";
            let font_size = 30.0;
            let text_dims = measure_text(text, None, font_size as u16, 1.0);
            let x = (screen_width() - text_dims.width) / 2.0;
            let y = screen_height() / 2.0;
            draw_text(text, x, y, font_size, WHITE);
        } else if started && points.len() > 1 {
            if step < 7 && get_time() > timer + 0.5 {
                tmp_points = chaikin(&points, step);
                step += 1;
                timer = get_time();
            }

            if step >= 7 {
                tmp_points = points.clone();
                timer = get_time();
                step = 0;
            }

            for i in 0..tmp_points.len() - 1 {
                let p1 = tmp_points[i];
                let p2 = tmp_points[i + 1];
                draw_line(p1.0, p1.1, p2.0, p2.1, 1.0, GREEN);
            }
        }
        next_frame().await;
    }
}