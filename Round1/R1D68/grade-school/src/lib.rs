use std::collections::HashMap;
pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let current_grade = self.grades.entry(grade).or_insert(Vec::new());
        current_grade.push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.grades.keys().map(|key| *key).collect::<Vec<u32>>();
        result.sort();
        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            Some(students) => {
                let mut result = students.clone();
                result.sort();
                result
            }
            None => Vec::new()
        }
    }
}
