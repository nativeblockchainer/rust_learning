use std::collections::{BTreeMap, HashSet};

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

impl Student {
    pub fn new(id: u64,name: String,age: u8,class: Class) -> Self {
        Student {
            id,
            name,
            age,
            class,
            courses: HashSet::new(),
            communities: HashSet::new()
        }
    }
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
#[derive(Debug,Clone, Eq, Hash, PartialEq)]
pub struct Course {
    pub name: String,//课程的名称
    pub students: Vec<u64>,//选择课程的学生id
}

/**
 * 社团元组结构体
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Community(pub String,pub Vec<u64>);

pub struct StudentManagementSystem {
    pub students: BTreeMap<u64, Student>,
    pub classes: BTreeMap<String, Class>,
    pub courses: BTreeMap<String, Course>,
    pub communities: BTreeMap<String,Community>,
}

impl StudentManagementSystem {
    pub fn new() -> Self {
        StudentManagementSystem {
            students: BTreeMap::new(),
            classes: BTreeMap::new(),
            courses: BTreeMap::new(),
            communities: BTreeMap::new(),
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

    // 新增 社团
    pub fn add_community(&mut self,community: Community) {
        self.communities.insert(community.0.clone(),community);
    }

    // 获取社区信息
    pub fn get_community(&self, name: &str) -> Option<&Community> {
        self.communities.get(name)
    }

    // 删除社团
    pub fn delete_community(&mut self, name: &str) -> Option<Community> {
        self.communities.remove(name)
    }

    // 学生选择一门课程
    pub fn select_course(&mut self, student_id: u64, course: &Course) {
        if let Some(ref mut student) = self.students.get_mut(&student_id) {
            student.courses.insert(course.clone());
        }
        if let Some(&mut ref mut course) = self.courses.get_mut(&course.name){
            course.students.push(student_id);
        }
    }

    // 学生加入一个社团
    pub fn join_community(&mut self, student_id: u64,community: &Community){
        if let Some(&mut ref mut student) = self.students.get_mut(&student_id) {
            student.communities.insert(community.clone());
        }
        if let Some(&mut ref mut community) = self.communities.get_mut(&community.0){
            community.1.push(student_id);
        }
    }
}