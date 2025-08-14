use egui_plot::{Legend, Line, Plot, PlotPoint, PlotPoints};
use std::time::Duration;
use thinkdsp::Sinusoid;

#[derive(Default)]
struct MyPlot {
    points: Vec<PlotPoint>,
}

impl eframe::App for MyPlot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("My Plot")
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui
                        .line(Line::new("curve", PlotPoints::Borrowed(&self.points)).name("curve"));
                });
        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    log::info!("Starting native…");
}

// TODO move all code related to gui to another module, add input

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Enable logs to the browser console:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok(); // TODO this doesnt work

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let sinusoid_signal = Sinusoid::new(440.0, 1.0, 0.0, f64::sin);

        let x_times = vec![
            Duration::from_secs_f64(0.0),
            Duration::from_secs_f64(0.25),
            Duration::from_secs_f64(0.5),
            Duration::from_secs_f64(0.75),
        ];
        let y_air_pressure = sinusoid_signal.evaluate(&x_times);

        let mut points: Vec<PlotPoint> = Vec::with_capacity(x_times.len());

        for i in 0..x_times.len() {
            let x = x_times[i].as_secs_f64();
            let y = y_air_pressure[i];
            points.push(PlotPoint::new(x, y));
        }
        // TODO use proper logging
        web_sys::console::log_1(&eframe::wasm_bindgen::JsValue::from_str(&format!(
            "{:?}",
            points
        )));

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_| Ok(Box::new(MyPlot { points }))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
