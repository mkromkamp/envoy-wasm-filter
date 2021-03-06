use log::info;
use log::trace;
use std::collections::HashMap;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext>{ Box::new(RequestCounterRoot)})
}

struct RequestCounterRoot;

impl Context for RequestCounterRoot {}
impl RootContext for RequestCounterRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(RequestCounter {
            context_id,
            header_key: "X-Requester".to_string(),
            num_requests: HashMap::default(),
        }))
    }
}

struct RequestCounter {
    context_id: u32,
    header_key: String,
    num_requests: HashMap<String, i64>,
}   

impl Context for RequestCounter {}

impl HttpContext for RequestCounter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        match self.get_http_request_header(&self.header_key) {
            Some(requester) => {
                // Get or create/insert current counter value
                let counter = self.num_requests
                    .entry(requester.to_string())
                    .and_modify(|c| { *c += 1})
                    .or_insert(1);

                info!("#{} #{} incremented to #{}", self.context_id, requester, counter);
                Action::Continue
            }
            _ => {
                info!("#{} no requester", self.context_id);
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
