//Timer
use std::time::SystemTime;
pub fn get_time() -> SystemTime {
    SystemTime::now()
}

pub fn get_time_taken(start_time: SystemTime) -> f32 {
    let end_time: SystemTime = SystemTime::now();
    match end_time.duration_since(start_time) {
        Ok(duration) => duration.as_secs_f32(),

        Err(e) => {
            panic!("Error: {:?}", e)
        }
    }
}
