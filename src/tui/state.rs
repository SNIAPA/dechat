pub struct State {
    pub messages: Vec<String>,
    pub url: String,
}

impl State {
    pub fn new(url: &str) -> Self {
        State {messages: vec![], url:url.to_string()}
    }
}
