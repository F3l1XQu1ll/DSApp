fn main() {
    // console_error_panic_hook::set_once();

    // tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebLogger::init(log::LevelFilter::Info).ok();
        eframe::WebRunner::new()
            .start(
                "dsapp_canvas",
                web_options,
                Box::new(|cc| Box::new(dsa_char_rs::DSApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
