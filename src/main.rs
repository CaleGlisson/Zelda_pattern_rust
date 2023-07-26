use rand::{thread_rng, Rng};
use slint::{Color, RgbaColor, SharedString, VecModel};
use std::rc::Rc;

fn make_circle(x: f32, y: f32, d: f32, background: Color, border: Color) -> Circle {
    Circle {
        x,
        y,
        d,
        background,
        border,
    }
}

fn make_color(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
    Color::from(RgbaColor {
        red,
        green,
        blue,
        alpha,
    })
}

fn draw_group(x: f32, y: f32, d: f32, group_color: Color) -> Vec<Circle> {
    let white: Color = make_color(1.0, 1.0, 1.0, 1.0);
    let no_color: Color = make_color(1.0, 1.0, 1.0, 0.0);
    const GAP: i32 = 40;
    let mut circles: Vec<Circle> = vec![];
    // Draw a white background in order to cover up what's under these
    circles.push(make_circle(x, y, d, white, no_color));

    let mut i = d as i32;
    while i > 0 {
        circles.push(make_circle(x, y, i as f32, no_color, group_color));
        i -= GAP;
    }

    circles
}
fn spline(points: Vec<(i32, i32)>) -> String {
    let size = points.len();
    let last = size - 2;
    let mut path = format!("M {} {}", points[0].0, points[0].1);

    for i in 0..size - 1 {
        let x0;
        let x1;
        let x2;
        let x3;
        let y0;
        let y1;
        let y2;
        let y3;

        if i == 0 {
            x0 = points[0].0;
            y0 = points[0].1;
        } else {
            x0 = points[i - 1].0;
            y0 = points[i - 1].1;
        }

        x1 = points[i].0;
        y1 = points[i].1;
        x2 = points[i + 1].0;
        y2 = points[i + 1].1;

        if i == last {
            x3 = x2;
            y3 = y2;
        } else {
            x3 = points[i + 2].0;
            y3 = points[i + 2].1;
        }

        let cp1x = x1 + ((x2 - x0) / 6);
        let cp1y = y1 + ((y2 - y0) / 6);
        let cp2x = x2 + ((x3 - x1) / 6);
        let cp2y = y2 + ((y3 - y1) / 6);

        path.push_str(&format!(
            " C {} {} {} {} {} {}",
            cp1x, cp1y, cp2x, cp2y, x2, y2
        ));
    }
    path
}
fn generate_squiggle_commands(x: i32, squiggle_width: i32, squiggle_height: i32) -> SharedString {
    let mut x_offset = squiggle_width / 2;
    let mut points: Vec<(i32, i32)> = vec![];
    let mut y = -squiggle_height;

    while y < 1000 + squiggle_height {
        points.push((x + x_offset, y));
        x_offset *= -1;
        y += squiggle_height;
    }
    slint::SharedString::from(spline(points))
}

fn main() {
    let main_window = MainWindow::new().unwrap();
    let model = Rc::new(VecModel::default());
    let squiggle = Rc::new(VecModel::default());
    let group_color = make_color(0.0, 1.0, 0.0, 1.0);
    let mut rng = thread_rng();
    let mut circles: Vec<Circle> = vec![];
    let mut lines: Vec<Squiggle> = vec![];
    main_window.set_model(model.clone().into());
    main_window.set_squiggle(squiggle.clone().into());

    for _ in 0..rng.gen_range(20..40) {
        circles.extend(draw_group(
            rng.gen_range(0..1000) as f32,
            rng.gen_range(0..1000) as f32,
            rng.gen_range(50..300) as f32,
            group_color,
        ));
    }

    let linegap: i32 = rng.gen_range(15..25);
    let squiggle_width = rng.gen_range(10..15);
    let squiggle_height = rng.gen_range(60..90);

    for x in (-linegap..(1000 + linegap)).step_by(linegap as usize) {
        lines.push(Squiggle {
            color: group_color,
            commands: generate_squiggle_commands(x, squiggle_width, squiggle_height),
        });
    }

    lines.push(Squiggle {
        color: group_color,
        commands: generate_squiggle_commands(10, 10, 20),
    });

    for circle in circles {
        model.push(circle);
    }

    for line in lines {
        squiggle.push(line)
    }
    main_window.run().unwrap();
}

slint::slint! {
    import { LineEdit, Button, Slider, StandardListView, VerticalBox } from "std-widgets.slint";

    struct Circle { x: length, y: length, d: length, background: color,
        border: color }

   struct Squiggle { color: color, commands: string }

    export component MainWindow inherits Window {
        preferred-width: 1000px;
        preferred-height: 1000px;

        in property <[Circle]> model;
        in property <[Squiggle]> squiggle;

        VerticalBox {
            Rectangle {
                background: white;
                border-color: black;
                border-width: 2px;
                clip: true;
                TouchArea {
                    width: 100%;
                    height: 100%;
                }

                for circle[idx] in root.model : Rectangle {
                    background: circle.background;
                    border-color: circle.border;
                    border-width: 10px;
                    border-radius: self.width / 2;
                    height: self.width;
                    width: circle.d;
                    x: circle.x - self.width/2;
                    y: circle.y - self.height/2;
                }
                for squiggle[idx] in root.squiggle : Path {
                    stroke: squiggle.color;
                    stroke-width: 10px;
                    commands: squiggle.commands;
                }
            }
        }
    }
}
