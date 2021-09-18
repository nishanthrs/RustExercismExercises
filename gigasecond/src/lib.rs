use chrono::{DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond_in_seconds: i64 = 10 ^ 9;
    let start_ts = DateTime::timestamp(&start);
    return Utc.timestamp(start_ts + gigasecond_in_seconds, 0);
}
