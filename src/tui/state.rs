pub struct State {
    pub messages: Vec<String>,
    pub url: String,
    pub hostname: String,
}

impl State {
    pub fn new(url: &str, hostname: &str) -> Self {
        State {
            messages: vec![],
            url: url.to_string(),
            hostname: hostname.to_string(),
        }
    }
}
