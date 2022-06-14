use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, Timelike};
use yaml_rust::{YamlEmitter, YamlLoader};
use yaml_rust::yaml::Yaml::Hash;
use crate::models::Daily;

pub fn remind() {
    let reminders: Vec<Daily> = create_reminders();
    loop {
        let date = Local::now();
        for x in reminders {
            let time = x.time;
            if time.hour() == date.hour() {
                if date.minute() - time.minute() < 5 {
                    fire(x).await;
                }
            }
        }
        sleep(Duration::from_secs(60))
    }
}

async fn fire(daily: Daily) {

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
    for m in &hash.keys() {
        let name = m.as_str().unwrap();
        let yaml = hash[name];
        let time = yaml["time"];
        let link = yaml["link"];
        reminders.push(Daily::new(name, time, link))
    }
    result
}