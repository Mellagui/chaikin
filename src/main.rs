use macroquad::prelude::*;

mod chaikin;
use chaikin::*;

#[macroquad::main("CHAIKIN")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut tmp_points: Vec<(f32, f32)> = Vec::new();
    let mut started = false;
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
        }
        if is_key_pressed(KeyCode::Enter) && points.len() > 1 {
            started = true
        }
        if is_mouse_button_pressed(MouseButton::Left) && !started {
            let (x, y) = mouse_position();
            points.push(( x, y ));
        }
        for point in points.iter() {
            draw_circle_lines(point.0, point.1, 5.0, 1. , RED);
        }

        if started {
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
                draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, YELLOW);
            }
        }

        next_frame().await;
    }
}