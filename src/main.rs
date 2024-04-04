use dotenv::dotenv;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let _ = dotenv();
    env_logger::init();
    let api_key = get_api_key();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .unwrap(),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Wallet checker",
        native_options,
        Box::new(|cc| Box::new(wallet_balance_gui::TemplateApp::new(cc, api_key))),
    )
}

fn get_api_key() -> String {
    std::env::var("API_KEY").expect("API_KEY is not set")
}

// // When compiling to web using trunk:
// #[cfg(target_arch = "wasm32")]
// fn main() {
//     // Redirect `log` message to `console.log` and friends:
//     eframe::WebLogger::init(log::LevelFilter::Debug).ok();
//     let api_key = get_api_key();

//     let web_options = eframe::WebOptions::default();

//     wasm_bindgen_futures::spawn_local(async {
//         eframe::WebRunner::new()
//             .start(
//                 "the_canvas_id", // hardcode it
//                 web_options,
//                 Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc, api_key))),
//             )
//             .await
//             .expect("failed to start eframe");
//     });
// }
