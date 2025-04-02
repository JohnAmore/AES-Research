//Timer

pub fn get_time() -> SystemTime {
    SystemTime::now();
}

pub fn get_time_taken(start_time: SystemTime) -> u32 {
    match end_time.duration_since(start_time) {
        Ok(duration) => duration.as_secs() as u32,

        Err(e) => {
            panic!("Error: {:?}", e)
        }
    }
}
