use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};

// TODO: render in the browser with WASM
// start from this removed code: https://github.com/magecnion/learning-dsp/commit/655f0b178502eb2562586b8713653eaa0082a192

#[derive(Default)]
struct MyPlot {
    plot_points: Vec<PlotPoint>,
}

impl eframe::App for MyPlot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("My Plot")
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("sin", PlotPoints::Borrowed(&self.plot_points)).name("sin"),
                    );
                });
        });
    }
}

pub fn render(times: Vec<f32>, samples: Vec<f32>) {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    log::info!("Starting native…");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 200.0]),
        ..Default::default()
    };

    let mut plot_points: Vec<PlotPoint> = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        let x = times[i];
        let y = samples[i];
        plot_points.push(PlotPoint::new(x, y));
    }

    if let Err(e) = eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Ok(Box::new(MyPlot { plot_points }))),
    ) {
        println!("Error rendering native: {e}");
        std::process::exit(1);
    }
}
