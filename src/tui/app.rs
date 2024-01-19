use std::sync::{Arc, Mutex};

use crate::tui::component::Component;

pub struct App {
    components: Vec<Arc<Mutex<dyn Component>>>
}
