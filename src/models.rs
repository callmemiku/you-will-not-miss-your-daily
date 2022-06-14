use chrono::{DateTime, Local, TimeZone};
use regex::Regex;

pub struct Daily {
    pub name: String,
    pub time: DateTime<Local>,
    pub link: String
}

impl Daily {
    pub fn new(name: String, time: String, link: String) -> Self {
        Self {
            name,
            time: get_time(time),
            link
        }
    }
}

fn get_time(time: String) -> DateTime<Local> {
    let pattern = Regex::new("[^0-9]").unwrap();
    let time_lower = time.to_lowercase();
    let mut time_slice: Vec<String> = time_lower.split(":").map(|n| n.to_string()).collect();
    let mut time_type = 0x0000;
    if time_lower.contains("am") || time.contains("pm") {
        time_type = time_type & 0x1000;
    };
    if time_slice.len() == 2 {
        time_type = time_type & 0x0010;
    }
    if time_slice.len() == 3 {
        time_type = time_type & 0x0020;
    }
    let hour: u32 = pattern.replace_all(&time_slice[0], "").parse::<u32>().unwrap();
    let minute: u32 = pattern.replace_all(&time_slice[1], "").parse::<u32>().unwrap();
    let second: u32;
    if time_type & 0x0020 == 0x0020 {
        second = pattern.replace_all(&time_slice[2], "").parse::<u32>().unwrap();
        time_type = time_type ^ 0x0020;
    } else {
        second = 0;
        time_type = time_type ^ 0x0010;
    }

    match time_type {
        0x0000 => Local.ymd( 0 ,0, 0).and_hms(hour, minute, second),
        0x1000 => {
            Local.ymd( 0 ,0, 0).and_hms(to_24(time_lower, hour), minute, second)
        },
        _ => {Local::now()}
    }
}

fn to_24(time: String, hour: u32) -> u32 {
    if time.contains("pm") {
        hour + 12
    } else {
        hour
    }
}