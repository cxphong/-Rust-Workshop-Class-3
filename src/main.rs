use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let mut school = School::new();
    school.add(10, "Katelin Hudson");
    school.add(20, "Rohaan Gilmore");
    school.add(20, "Zac Power");
    school.add(10, "Nico Benitez");
    school.add(30, "Nala Vance");
    school.add(100, "Sydney Doyle");
    school.add(20, "Tony Booth");
    let vec1 = school.grades();
    let vec2 = school.grade(20);
    
    println!("vec1 = {:?}, vec2 = {:?}", vec1, vec2);
}

pub struct School {
    students: HashMap<String, u32>
}

impl School {

    pub fn new() -> School {
        Self {students: HashMap::new()}
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<&u32> {
        let mut set = HashSet::new();
        for value in self.students.values() {
            set.insert(value);
        }
        
        let mut vec = Vec::from_iter(set);
        vec.sort_by(|a, b| a.cmp(b));
        return vec;
    }

    pub fn grade(&self, grade: u32) -> Vec<&String> {
        let mut student = Vec::new();
        
        for (key, val) in self.students.iter() {
            if *val == grade {
                student.push(key);
            }
        }
        
        student.sort_by(|a, b| a.cmp(b));
        return student;
    }
    
}










