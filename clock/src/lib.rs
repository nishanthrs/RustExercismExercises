use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn normalize_hours(hours: i32) -> i32 {
        let mut normalized_hours: i32 = hours % 24;
        if normalized_hours < 0 {
            normalized_hours = 24 - normalized_hours.abs();
        }
        return normalized_hours;
    }

    pub fn normalize_minutes(minutes: i32) -> i32 {
        let mut normalized_minutes: i32 = minutes % 60;
        if normalized_minutes < 0 {
            normalized_minutes = 60 - normalized_minutes.abs();
        }
        return normalized_minutes;
    }

    pub fn compute_hours_to_add(minutes: i32) -> i32 {
        let hours_to_add = minutes / 60;
        if minutes < 0 && minutes % 60 != 0 {
            return hours_to_add - 1;
        }
        return hours_to_add;
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_minutes: i32 = Clock::normalize_minutes(minutes);
        let normalized_hours: i32 = Clock::normalize_hours(hours + Clock::compute_hours_to_add(minutes));

        return Clock {
            hours: normalized_hours,
            minutes: normalized_minutes,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let raw_minutes: i32 = self.minutes + minutes;
        let normalized_minutes: i32 = Clock::normalize_minutes(raw_minutes);
        let normalized_hours: i32 = Clock::normalize_hours(
            self.hours + Clock::compute_hours_to_add(raw_minutes)
        );

        return Clock {
            hours: normalized_hours,
            minutes: normalized_minutes,    
        };
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{:02}:{:02}", self.hours, self.minutes);
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}
