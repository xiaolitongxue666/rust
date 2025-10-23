use std::collections::{BTreeSet, HashMap};

pub struct School {
    students: HashMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student_string = student.to_string();

        // 检查学生是否已经在其他年级
        for (existing_grade, students) in &self.students {
            if students.contains(&student_string) && *existing_grade != grade {
                return; // 如果学生已经在其他年级，不添加
            }
        }

        // 添加到指定年级（BTreeSet自动去重）
        self.students
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(student_string);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.keys().cloned().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.students
            .get(&grade)
            .map(|students| students.iter().cloned().collect())
            .unwrap_or_default()
    }
}
