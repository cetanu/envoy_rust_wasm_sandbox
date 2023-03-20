use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

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
    fn on_http_response_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {
        let val = self.get_property(vec![
            "metadata",
            "filter_metadata",
            "envoy.filters.network.rbac",
            "something",
        ]);
        info!("{val:?}");
        self.set_property(
            vec![
                "metadata",
                "filter_metadata",
                "envoy.filters.network.rbac",
                "something",
            ],
            Some(b"value"),
        );
        Action::Continue
    }
}
