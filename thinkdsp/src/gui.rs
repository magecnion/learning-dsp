use std::{env, fs::create_dir_all};

use plotters::prelude::*;

// TODO: render in the browser with WASM
// start from this removed code: https://github.com/magecnion/learning-dsp/commit/655f0b178502eb2562586b8713653eaa0082a192

// #[derive(Default)]
// struct MyPlot {
//     plot_points: Vec<PlotPoint>,
// }

// impl eframe::App for MyPlot {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             Plot::new("My Plot")
//                 .legend(Legend::default())
//                 .show(ui, |plot_ui| {
//                     plot_ui.line(
//                         Line::new("sin", PlotPoints::Borrowed(&self.plot_points)).name("sin"),
//                     );
//                 });
//         });
//     }
// }

// pub fn render(times: Vec<f32>, samples: Vec<f32>) {
//     env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
//     log::info!("Starting native…");

//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 200.0]),
//         ..Default::default()
//     };

//     let mut plot_points: Vec<PlotPoint> = Vec::with_capacity(times.len());
//     for i in 0..times.len() {
//         let x = times[i];
//         let y = samples[i];
//         plot_points.push(PlotPoint::new(x, y));
//     }

//     if let Err(e) = eframe::run_native(
//         "My egui App with a plot",
//         options,
//         Box::new(|_cc| Ok(Box::new(MyPlot { plot_points }))),
//     ) {
//         println!("Error rendering native: {e}");
//         std::process::exit(1);
//     }
// }

const OUTPUT_DIR: &str = "./plots";

pub fn draw(
    filename: Option<String>,
    ts: Vec<f32>,
    ys: Vec<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(OUTPUT_DIR)?;

    let binary_name = env::current_exe()?
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let filename = match filename {
        Some(ref name) => format!("{}_{}", binary_name, name),
        None => binary_name.clone(),
    };
    let file_path = format!("{}/{}.png", OUTPUT_DIR, filename,);
    log::debug!("plot saved in: {}", file_path);

    let root = BitMapBackend::new(&file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate the range for the plot
    let x_min = ts.first().unwrap_or(&0.0);
    let x_max = ts.last().unwrap_or(&1.0);
    let y_min = ys.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let y_max = ys.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    let mut chart = ChartBuilder::on(&root)
        .caption(filename, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(*x_min..*x_max, y_min..y_max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        ts.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}
