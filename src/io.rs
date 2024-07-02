use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Input {
    pub(crate) source: String,
    pub(crate) filesystem: HashMap<String, Vec<u8>>,
    #[serde(rename = "dont-fail")]
    pub(crate) dont_fail: bool,
}

#[derive(Serialize)]
pub(crate) struct Output {
    pub(crate) pages: Option<Vec<String>>,
    pub(crate) error: Option<String>,
}

impl Output {
    pub fn new(pages: Vec<String>) -> Self {
        Output {
            pages: Some(pages),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Output {
            pages: None,
            error: Some(error),
        }
    }
}
