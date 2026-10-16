pub struct Message {
    title: String,
    body: String,
    command: String,

}

impl Message {
    pub fn new(title: &str, body: &str, command: &str) -> Self {
        Self { title: title.to_string(), body: body.to_string(), command: command.to_string() }
    }
}
