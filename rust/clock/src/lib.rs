use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut seconds = minutes * 60;
        seconds = seconds + (hours * 3600);
        if seconds.is_negative() {
            let wraps = (-seconds / 86400) + 1;
            seconds = (86400 * wraps) + seconds;
        }
        let hours = (seconds / 3600) % 24;
        seconds = seconds - (hours * 3600);
        let minutes = (seconds / 60) % 60;
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut seconds = (self.minutes + minutes) * 60;
        seconds = seconds + (self.hours * 3600);
        let hours = (seconds / 3600) % 24;
        seconds = seconds - (hours * 3600);
        let minutes = (seconds / 60) % 60;
        Clock::new(hours, minutes)
    }
}
