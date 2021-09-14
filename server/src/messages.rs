use chrono::{DateTime, Utc};

pub struct Message {
    pub title: String,
    pub body: String,
}

pub fn generate_message() -> Message {
    Message {
        title: "Hello".to_string(),
        body: format! {"Hello world! It's {}", current_date_time()},
    }
}

fn current_date_time() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
