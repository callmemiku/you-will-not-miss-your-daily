use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, Timelike};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use yaml_rust::yaml::Yaml::Hash;
use crate::models::Daily;
use soloud::*;

pub fn remind() {
    let reminders: Vec<Daily> = create_reminders();
    loop {
        let date = Local::now();
        for x in &reminders {
            let time = x.time;
            if time.hour() == date.hour() {
                if date.minute() - time.minute() < 5 {
                    fire(x);
                }
            }
        }
        sleep(Duration::from_secs(60))
    }
}

fn fire(daily: &Daily) {
    println!("{}", daily.name);
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("../resources/notification.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn create_reminders() -> Vec<Daily> {
    let mut reminders: Vec<Daily> = vec![];
    let mut path = std::env::current_dir().unwrap();
    path.push("resources");
    path.push("reminders");
    path.set_extension("yml");
    let result = vec![];
    let contents = std::fs::read_to_string(path).unwrap();
    let data = &YamlLoader::load_from_str(&contents).unwrap()[0]["reminders"];
    let hash = data.as_hash().unwrap();
    for m in hash.keys() {
        let yaml: &Yaml = &hash[m];
        let time = &yaml["time"];
        let link = &yaml["link"];
        reminders.push(Daily::new(m.as_str().unwrap().parse().unwrap(), time.as_str().unwrap().parse().unwrap(), link.as_str().unwrap().parse().unwrap()))
    }
    result
}