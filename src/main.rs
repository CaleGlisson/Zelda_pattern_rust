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
    let mut path = format!("M {},{}", points[0].0, points[0].1);

    for i in 0..size - 1 {
        let x0 = if i == 0 { points[0].0 } else { points[i - 1].0 };
        let y0 = if i == 0 { points[0].1 } else { points[i - 1].1 };

        let x1 = points[i].0;
        let y1 = points[i].1;
        let x2 = points[i + 1].0;
        let y2 = points[i + 1].1;

        let x3 = if i == last { x2 } else { points[i + 2].0 };
        let y3 = if i == last { y2 } else { points[i + 2].1 };

        let cp1x = x1 + ((x2 - x0) / 6);
        let cp1y = y1 + ((y2 - y0) / 6);
        let cp2x = x2 - ((x3 - x1) / 6);
        let cp2y = y2 - ((y3 - y1) / 6);

        path.push_str(&format!(
            " C {},{} {},{} {},{}",
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

    lines.push(Squiggle {
        spacing: linegap as f32,
        squiggle_width: squiggle_width as f32,
        color: group_color,
        commands: generate_squiggle_commands(0, squiggle_width, squiggle_height),
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

   struct Squiggle { spacing: length, squiggle_width: length, color: color, commands: string }

    export component MainWindow inherits Window {
        preferred-width: 1000px;
        preferred-height: 1000px;

        in property <[Circle]> model;
        in property <[Squiggle]> squiggle;


        private property<int> iters: root.width / root.squiggle[0].squiggle_width;

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

                for i in iters : Path {
                    x: (i * (root.squiggle[0].squiggle_width + root.squiggle[0].spacing))
                         - (root.width / 2);
                    stroke: root.squiggle[0].color;
                    stroke-width: 10px;
                    commands: root.squiggle[0].commands;
                    viewbox-x: 0;
                    viewbox-y: 0;
                    clip: true;
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
            }
        }
    }
}
