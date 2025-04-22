use chrono::Duration;
use events::Event::*;
#[allow(unused_imports)]
use events::{Notification, Position};

#[allow(dead_code)]
fn main() {
    let remainder = Remainder("Go to the doctor");
    println!("{}", remainder.notify());
    let registration = Registration(Duration::seconds(49094));
    println!("{}", registration.notify());
    let appointment = Appointment("Go to the doctor");
    println!("{}", appointment.notify());
    let holiday = Holiday;
    println!("{}", holiday.notify());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn remainder_notification() {
        let remainder = Remainder("Go to the doctor");
        let notification = remainder.notify();
        println!("{}", &notification);
        assert_eq!(
            notification,
            Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: "Go to the doctor".to_string(),
            }
        );
    }

    #[test]
    fn registration_notification() {
        let registration = Registration(Duration::seconds(49094));
        let notification = registration.notify();
        println!("{}", registration.notify());
        assert_eq!(
            notification,
            Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: "You have 13H:38M:14S left before the registration ends".to_string(),
            }
        );
    }

    #[test]
    fn appointment_notification() {
        let appointment = Appointment("Go to the doctor");
        let notification = appointment.notify();
        println!("{}", &notification);
        assert_eq!(
            notification,
            Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: "Go to the doctor".to_string(),
            }
        );
    }

    #[test]
    fn holiday_notification() {
        let holiday = Holiday;
        let notification = Holiday.notify();
        println!("{}", holiday.notify());
        assert_eq!(
            notification,
            Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: String::from("Enjoy your holiday"),
            }
        );
    }
}
