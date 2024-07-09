use once_cell::sync::Lazy;

#[allow(dead_code)]
pub const TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJtYXRoaWV1bGVicmFhYXNAZ21haWwuY29tIiwiaWF0IjoxNzExMTI0MzQxLCJleHAiOjE3NjI5NjQzNDEsInJvbGUiOiJ1c2VyIn0.OfP32SVlG0XcV5Pf-LIJt9T6j1g0cCFaUnW00k3dL1w";
#[allow(dead_code)]
pub const BAD_TOKEN_FOR_TEST: &str = "Bearer eyJ0eXAiOiJKV1QiLCJiOiJIUzI1NiJ9.eyJzdWIiOiJtYXRoaWV1bGVicmFhYXNAZ21haWwuY29tIiwiaWF0IjoxNzExMTI0MzQxLCJleHAiOjE3NjbGUiOiJ1c2VyIn0.OfP32SVlG0XcV5Pf-LIJt9T6j1g0cCFa";

pub const CONFIG: Lazy<shared::config::Config> = Lazy::new(|| shared::config::Config::init());
