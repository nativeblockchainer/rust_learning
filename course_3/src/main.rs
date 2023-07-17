mod student_manage_sys;

pub use student_manage_sys::{Student,Course,Class,StudentManagementSystem};
use std::collections::HashSet;

fn main() {
    let mut sms: StudentManagementSystem = StudentManagementSystem::new();

    let student1: Student = Student {
        id: 1,
        name: "Alice".to_string(),
        class_name: "Class A".to_string(),
        courses: HashSet::new(),
    };

    let student2 = Student {
        id: 2,
        name: "Bob".to_string(),
        class_name: "Class B".to_string(),
        courses: HashSet::new(),
    };

    let class1 = Class {
        name: student1.class_name.clone(),
        students: vec![student1.id],
    };

    let class2 = Class {
        name: student2.class_name.clone(),
        students: vec![student2.id],
    };

    let course1 = Course {
        name: "Math".to_string(),
        students: HashSet::new(),
    };

    let course2 = Course {
        name: "English".to_string(),
        students: HashSet::new(),
    };

    sms.add_student(student1);
    sms.add_student(student2);

    sms.add_class(class1);
    sms.add_class(class2);

    sms.add_course(course1.clone());
    sms.add_course(course2.clone());

    if let Some(student) = sms.get_student(1) {
        let mut updated_student: Student = student.clone();
        updated_student.courses.insert(course1.name.clone());
        updated_student.courses.insert(course2.name.clone());
        sms.add_student(updated_student);
    }

    if let Some(course) = sms.get_course("Math") {
        let mut updated_course: Course = course.clone();
        updated_course.students.insert(student1.id);
        sms.add_course(updated_course);
    }

    println!("Students: {:?}", sms.students);
    println!("Classes: {:?}", sms.classes);
    println!("Courses: {:?}", sms.courses);

    sms.delete_student(2);
    sms.delete_class("Class A");
    sms.delete_course("English");

    println!("After deletion:");
    println!("Students: {:?}", sms.students);
    println!("Classes: {:?}", sms.classes);
    println!("Courses: {:?}", sms.courses);
}
