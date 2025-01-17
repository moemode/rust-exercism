use std::{collections::HashSet, sync::Mutex};
use lazy_static::lazy_static;
use rand::Rng;

pub struct Robot(String);

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

impl Robot {
    pub fn new() -> Self {
        let mut r = Robot("".into());
        r.assign_unique_name();
        r
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        let letter1 = (rng.gen_range('A'..='Z')) as char;
        let letter2 = (rng.gen_range('A'..='Z')) as char;
        let digits = rng.gen_range(0..1000);
        format!("{}{}{:03}", letter1, letter2, digits)
    }

    fn assign_unique_name(&mut self) {
        let mut unique = Self::generate_name();
        let mut set = USED_NAMES.lock().unwrap();
        while set.contains(&unique) {
            unique = Self::generate_name();
        }
        set.insert(unique.clone());
        self.0 = unique;
    }

    pub fn reset_name(&mut self) {
        let old_name = self.0.clone();
        self.assign_unique_name();
        let mut set = USED_NAMES.lock().unwrap();
        set.remove(&old_name);
    }

    pub fn name(&self) -> &str {
        &self.0
    }

}
