use std::collections::{HashMap, HashSet};

#[derive(Debug,Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub class_name: String,
    pub courses: HashSet<String>,
}

#[derive(Debug,Clone)]
pub struct Class {
    pub name: String,
    pub students: Vec<u32>,
}

#[derive(Debug,Clone)]
pub struct Course {
    pub name: String,
    pub students: HashSet<u32>,
}

pub struct StudentManagementSystem {
    pub students: HashMap<u32, Student>,
    pub classes: HashMap<String, Class>,
    pub courses: HashMap<String, Course>,
}

impl StudentManagementSystem {
    pub fn new() -> Self {
        StudentManagementSystem {
            students: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
        }
    }

    // 添加学生
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    // 获取学生信息
    pub fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    // 删除学生
    pub fn delete_student(&mut self, id: u32) -> Option<Student> {
        self.students.remove(&id)
    }

    // 添加班级
    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.name.clone(), class);
    }

    // 获取班级信息
    pub fn get_class(&self, name: &str) -> Option<&Class> {
        self.classes.get(name)
    }

    // 删除班级
    pub fn delete_class(&mut self, name: &str) -> Option<Class> {
        self.classes.remove(name)
    }

    // 添加课程
    pub fn add_course(&mut self, course: Course) {
        self.courses.insert(course.name.clone(), course);
    }

    // 获取课程信息
    pub fn get_course(&self, name: &str) -> Option<&Course> {
        self.courses.get(name)
    }

    // 删除课程
    pub fn delete_course(&mut self, name: &str) -> Option<Course> {
        self.courses.remove(name)
    }
}