use piston_window::*;
use rayon_logs::visualisation;
use rayon_logs::RunLog;

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

    let mut window: PistonWindow = WindowSettings::new("rayon logs viewer", [600, 600])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    println!("size: {:?}", window.size());

    window.set_lazy(true);
    while let Some(e) = window.next() {
        let size = window.size();
        let width = size.width;
        let height = size.height;
        window.draw_2d(&e, |c, g| {
            clear([0.4, 0.4, 0.4, 1.0], g);
            g.clear_stencil(0);

            for r in &scene.rectangles {
                let scale = c
                    .transform
                    .trans(-xmin, -ymin)
                    .scale(width / (xmax - xmin), height / (ymax - ymin));
                Rectangle::new([r.color[0], r.color[1], r.color[2], 1.0]).draw(
                    [r.x, r.y, r.width, r.height],
                    &c.draw_state,
                    scale,
                    g,
                );
            }
        });
    }
}
