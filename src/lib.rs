use log::trace;
use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let header_key = "X-Correlation-ID";

        match self.get_http_request_header(header_key) {
            Some(_) => {
                info!("#{} Found header", self.context_id);
                Action::Continue
            }
            _ => {
                let header_value = "abcd";

                self.set_http_request_header(header_key, Some(header_value));
                info!("#{} Added header", self.context_id);
                Action::Continue
            }
        }
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            trace!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}