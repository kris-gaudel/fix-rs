#[derive(Debug)]
pub struct Field {
    tag: u32,
    value: String,
}

#[derive(Debug)]
pub struct FixMessage {
    header: Vec<Field>,
    body: Vec<Field>,
    trailer: Vec<Field>,
    msg_type: String,
}

impl Field {
    pub fn new(tag: u32, value: String) -> Field {
        Field { tag, value }
    }
}

impl FixMessage {
    pub fn new() -> FixMessage {
        FixMessage {
            header: Vec::new(),
            body: Vec::new(),
            trailer: Vec::new(),
            msg_type: String::new(),
        }
    }
}
