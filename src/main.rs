use chrono::{Local, Timelike, Utc};

fn minutes_in_day(hour: u32, minute: u32) -> u32 {
    1440 - ((hour * 60) + minute)
}

#[allow(dead_code)]
fn custom_range() -> (u32, u32) {
    return (0, 0)
}

fn dawn_to_midnight() -> (u32, u32) {
    let timezone_offset = (((Local::now().offset().local_minus_utc()) * -1)/60)/60;
    let now = Utc::now();
    let hour = now.hour() - (timezone_offset as u32);
    let minute = now.minute();

    return (hour, minute)
}

fn main() {
    let (hour, minute) = dawn_to_midnight();
    println!("{} left in day!", minutes_in_day(hour, minute));
}


