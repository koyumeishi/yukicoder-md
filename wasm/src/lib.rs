mod utils;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::sync::Once;
        use wasm_bindgen::prelude::*;
        use wasm_timer::Instant;
        use log::{info, Level};
        // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
        // allocator.
        #[cfg(feature = "wee_alloc")]
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

        static INIT_LOGGER: Once = Once::new();

        #[wasm_bindgen]
        pub fn convert(input: String, use_template_engine: bool) -> String {
            INIT_LOGGER.call_once(|| {init();});
            let start_time = Instant::now();
            let res = yukicoder_md_lib::convert(input, false, use_template_engine);
            let end_time = Instant::now();
            info!("conversion completed in {:?}", end_time.duration_since(start_time));

            res
        }

        fn init(){
            utils::set_panic_hook();
            console_log::init_with_level(Level::Info).expect("error initializing log");
        }

    } else {
        // skip
    }
}
