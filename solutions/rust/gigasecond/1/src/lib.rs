use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let time = start + Duration::seconds(1000000000);
    println!("What time is a gigasecond later than {}?", start);
    println!("Time after one gigasecond is {}.", time);
    time
}
