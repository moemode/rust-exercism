use core::panic;
use rand::Rng;
use std::cell::RefCell;

pub struct Robot(String);

thread_local! {
    static AVAILABLE_NAMES: RefCell<Vec<String>> = {
        let mut v = Vec::with_capacity(26*26*1000);
        for ch1 in 'A'..='Z' {
            for ch2 in 'A'..='Z' {
                for ch3 in '0'..='9' {
                    for ch4 in '0'..='9' {
                        for ch5 in '0'..='9' {
                            let name = format!("{}{}{}{}{}", ch1,ch2,ch3,ch4,ch5);
                            v.push(name);
                        }
                    }
                }
            }
        }
        RefCell::new(v)
    };
}

impl Robot {
    pub fn new() -> Self {
        let mut r = Robot("".into());
        r.assign_unique_name();
        r
    }

    fn assign_unique_name(&mut self) {
        self.0 = AVAILABLE_NAMES.with_borrow_mut(|names| {
            if names.len() == 0 {
                panic!("No more names available");
            }
            let name_index = rand::thread_rng().gen_range(0..names.len());
            names.swap_remove(name_index)
        });
    }

    pub fn reset_name(&mut self) {
        let old_name = self.0.clone();
        self.assign_unique_name();
        AVAILABLE_NAMES.with_borrow_mut(|names| {
            names.push(old_name);
        });
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}
