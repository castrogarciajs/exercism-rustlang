use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes) % (24 * 60);
        Clock {
            minutes: if total_minutes >= 0 {
                total_minutes
            } else {
                total_minutes + 24 * 60
            },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
fn main() {
    /*
        Instructions
    Implement a clock that handles times without dates.

    You should be able to add and subtract minutes to it.

    Two clocks that represent the same time should be equal to each other.

    You will also need to implement .to_string() for the Clock struct. We will be using this to display the Clock's state. You can either do it via implementing it directly or using the Display trait.

    Did you implement .to_string() for the Clock struct?

    If so, try implementing the Display trait for Clock instead.

    Traits allow for a common way to implement functionality for various types.

    For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display, which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.
         */
    println!("Hello, world!");
}
