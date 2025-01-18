use std::{cell::{RefCell}, collections::HashSet };
use rand::Rng;

pub struct Robot(String);

thread_local! {
    static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
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
        USED_NAMES.with_borrow_mut(|set| {
            while set.contains(&unique) {
                unique = Self::generate_name();
            }
            set.insert(unique.clone());
        });
        self.0 = unique;
    }

    pub fn reset_name(&mut self) {
        let old_name = self.0.clone();
        self.assign_unique_name();
        USED_NAMES.with_borrow_mut(|set| set.remove(&old_name));
    }

    pub fn name(&self) -> &str {
        &self.0
    }

}
