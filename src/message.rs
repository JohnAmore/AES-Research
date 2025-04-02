// Message
// Contains the content of the data, the index of where the state box is at, and the file name.

mod reader;
struct Message {
    content: String,
    at: u32,
    filename: String,
}

impl Message {
    pub fn create(file: String) -> Message {
        let mut message: String = String::new();
        let fcontent = reader::read_message(file);
        Message {
            content = fcontent,
            at = 0,
            filename = file,
        }
    }
}
