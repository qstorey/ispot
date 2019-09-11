use chrono::prelude::*;

pub fn datetime_to_string() -> String {
    let local_time: DateTime<Local> = Local::now();
    local_time.format("%Y-%m-%d %H:%M:%S").to_string()
}
