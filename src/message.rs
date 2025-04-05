// Message
// Contains the content of the data, the index of where the state box is at, and the file name.
use crate::reader;
pub struct Message {
    pub content: String,
    pub at: u32,
    pub filename: String,
}

impl Message {
    pub fn create(file: String) -> Message {
        assert!(!file.is_empty(), "String should not be empty");
        let mut message: String = String::new();
        let fcontent = reader::read_message(file.clone());
        Message {
            content: fcontent,
            at: 0,
            filename: file,
        }
    }
}
