use time::{ PrimitiveDateTime as DateTime, SignedDuration as Duration };

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let time = start + Duration::seconds(1_000_000_000);
    println!("What time is a gigasecond later than {}?", start);
    println!("Time after one gigasecond is {}.", time);
    time
}
