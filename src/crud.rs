use std::collections::HashMap;
use super::models::*;


pub trait Postable {
    fn create(&self, body: HashMap<String, String>) -> Option<String>;
}

impl Postable for Author {
    fn create(&self, body: HashMap<String, String>) -> Option<String> {
        tracing::info!("Received body {:?}", body);
        Some("Got a JSON body!".to_string())
    }
}

