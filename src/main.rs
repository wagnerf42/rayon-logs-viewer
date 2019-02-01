use piston_window::*;
use rayon_logs::visualisation;
use rayon_logs::RunLog;

fn draw_segment(s: &((f64, f64), (f64, f64)), c: &Context, g: &mut G2d, scale: [[f64; 3]; 2]) {
    Line::new([0.0, 0.0, 0.0, 1.0], 0.1).draw(
        [(s.0).0, (s.0).1, (s.1).0, (s.1).1],
        &c.draw_state,
        scale,
        g,
    );
}

fn draw_rectangle(
    r: &rayon_logs::Rectangle,
    c: &Context,
    g: &mut G2d,
    scale: [[f64; 3]; 2],
    current_time: u64,
) {
    Rectangle::new([0.0, 0.0, 0.0, 1.0]).draw(
        [r.x, r.y, r.width, r.height],
        &c.draw_state,
        scale,
        g,
    );
    let time_scale = r
        .animation
        .map(|(start, end)| {
            if current_time < start {
                0.0
            } else if current_time >= end {
                1.0
            } else {
                (current_time - start) as f64 / (end - start) as f64
            }
        })
        .unwrap_or(1.0);
    Rectangle::new([r.color[0], r.color[1], r.color[2], 1.0]).draw(
        [r.x, r.y, r.width * time_scale, r.height * time_scale],
        &c.draw_state,
        scale,
        g,
    );
}

fn main() {
    let log = RunLog::load("max.json").expect("loading log failed");
    let scene = visualisation(&log, None);
    let xmax = scene
        .rectangles
        .iter()
        .map(|r| r.width + r.x)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let ymax = scene
        .rectangles
        .iter()
        .map(|r| r.height + r.y)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let xmin = scene
        .rectangles
        .iter()
        .map(|r| r.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let ymin = scene
        .rectangles
        .iter()
        .map(|r| r.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_time = scene
        .rectangles
        .iter()
        .map(|r| r.animation.unwrap().0)
        .min()
        .unwrap();
    let max_time = scene
        .rectangles
        .iter()
        .map(|r| r.animation.unwrap().1)
        .max()
        .unwrap();

    let mut window: PistonWindow = WindowSettings::new("rayon logs viewer", [600, 600])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    window.set_lazy(true);
    let mut current_time = (min_time + max_time) / 2;
    while let Some(e) = window.next() {
        let size = window.size();
        let width = size.width;
        let height = size.height;
        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            g.clear_stencil(0);

            let scale = c
                .transform
                .trans(-xmin, -ymin)
                .scale(width / (xmax - xmin), height / (ymax - ymin));

            for s in &scene.segments {
                draw_segment(s, &c, g, scale);
            }
            for r in &scene.rectangles {
                draw_rectangle(r, &c, g, scale, current_time);
            }
        });
    }
}
