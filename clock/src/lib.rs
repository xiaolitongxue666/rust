use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {

        println!(
            "Construct a new Clock from {:02} : {:02}",
            hours,
            minutes
        );

        let divisor = minutes.div_euclid(60);
        let remainder = minutes.rem_euclid(60);

        println!("Minutes div 60, divisor is {}, remainder  is {} .", divisor, remainder );
        Self {
            hours : (hours + divisor).rem_euclid(24),
            minutes : remainder,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        println!("Add {} minutes to existing Clock time", minutes);
        let divisor = (self.minutes + minutes).div_euclid(60);
        let remainder = (self.minutes + minutes).rem_euclid(60);

        println!("Add minutes div 60, divisor is {}, remainder  is {} .", divisor, remainder );

        Self {
            hours : (self.hours + divisor).rem_euclid(24),
            minutes : remainder,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:02}:{:02}", self.hours, self.minutes)
    }
}

