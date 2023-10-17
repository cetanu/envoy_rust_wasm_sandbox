use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::str::from_utf8;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(WasmFilter) });
}}
struct WasmFilter;
impl Context for WasmFilter {}
impl RootContext for WasmFilter {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, _context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(WasmFilter))
    }
}

impl HttpContext for WasmFilter {
    fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // dynamic metadata
        self.set_property(
            vec![
                "metadata",
                "filter_metadata",
                "envoy.filters.http.wasm",
                "something",
            ],
            Some(b"value"),
        );
        info!("Set property to 'value'");

        // shared data
        let _ = self.set_shared_data("example", Some(b"shared"), None);

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        // dynamic metadata
        let val = self
            .get_property(vec![
                "metadata",
                "filter_metadata",
                "envoy.filters.http.wasm",
                "something",
            ])
            .unwrap_or("Nothing".into());
        let s = from_utf8(val.as_ref()).unwrap_or("Nothing!");
        info!("Got property: {s}");
        self.set_http_response_header("x-dynamic-value", Some(s));

        // shared data
        let (shared, _) = self.get_shared_data("example");
        let shared_data = shared.unwrap();
        let s2 = from_utf8(&shared_data).unwrap_or("Nothing!");
        self.set_http_response_header("x-shared-value", Some(s2));

        Action::Continue
    }
}
