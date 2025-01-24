use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MINUTES_PER_DAY: i32 = 24 * 60;
    const MINUTES_PER_HOUR: i32 = 60;

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * Self::MINUTES_PER_HOUR + minutes;
        let minutes = total_minutes.rem_euclid(Self::MINUTES_PER_DAY);
        Clock {
            hours: minutes / 60,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
