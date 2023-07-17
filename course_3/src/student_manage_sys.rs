use std::collections::{HashMap, HashSet};

/**
 * 学生结构体
 */
#[derive(Debug,Clone)]
pub struct Student {
    pub id: u64,//ID
    pub name: String,//姓名
    pub age: u8,//年龄
    pub class: Class,//所在班级
    pub courses: HashSet<Course>,//选择的课程
    pub communities: HashSet<Community>,//加入的社团
}

/**
 * 班级结构体
 */
#[derive(Debug,Clone)]
pub struct Class {
    pub name: String,//班级名称
    pub students: Vec<u64>,//班级的学生id
}

/**
 * 课程结构体
 */
#[derive(Debug,Clone)]
pub struct Course {
    pub name: String,//课程的名称
    pub students: HashSet<u64>,//选择课程的学生id
}

/**
 * 社团元组结构体
 */
#[derive(Debug,Clone)]
pub struct Community(String,HashSet<u64>);

pub struct StudentManagementSystem {
    pub students: HashMap<u64, Student>,
    pub classes: HashMap<String, Class>,
    pub courses: HashMap<String, Course>,
    pub communities: HashMap<String,Community>,
}

impl StudentManagementSystem {
    pub fn new() -> Self {
        StudentManagementSystem {
            students: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
            communities: HashMap::new(),
        }
    }

    // 添加学生
    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    // 获取学生信息
    pub fn get_student(&self, id: u64) -> Option<&Student> {
        self.students.get(&id)
    }

    // 删除学生
    pub fn delete_student(&mut self, id: u64) -> Option<Student> {
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
        let if Option(c: &Class) = self.classes.get(name){
            
        }
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