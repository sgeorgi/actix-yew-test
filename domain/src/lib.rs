pub struct Message {
    pub title: String,
    pub body: String,
}

pub fn generate_message() -> Message {
    Message {
        title: "Hello".to_string(),
        body: "Hello world!".to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
