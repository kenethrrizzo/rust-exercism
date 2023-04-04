use core::fmt;

#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::normalize_time(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::normalize_time(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    fn normalize_time(mut hours: i32, mut minutes: i32) -> Self {
        if minutes >= 60 {
            hours += minutes / 60;
            minutes = minutes % 60;
        }
        while minutes < 0 {
            minutes += 60;
            hours -= 1;
        }
        if hours >= 24 {
            hours = hours % 24;
        }
        while hours < 0 {
            hours += 24;
        }
        Clock { hours, minutes }
    }
}
