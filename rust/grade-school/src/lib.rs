use std::collections::{BTreeMap, BTreeSet, HashSet};

pub struct School {
    all_students: HashSet<String>,
    grade_students: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            all_students: HashSet::new(),
            grade_students: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.all_students.contains(student) {
            println!("Student already exists");
            return;
        }
        self.all_students.insert(student.into());
        self.grade_students
            .entry(grade)
            .or_default()
            .insert(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grade_students.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grade_students
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }

    ///Get a sorted list of all students in all grades. Grades should sort as 1, 2, 3, etc.,
    /// and students within a grade should be sorted alphabetically by name.
    pub fn sorted_students(&self) -> Vec<String> {
        self.grade_students
            .keys()
            .flat_map(|g| self.grade_students.get(g).unwrap())
            .cloned()
            .collect()
    }
}
