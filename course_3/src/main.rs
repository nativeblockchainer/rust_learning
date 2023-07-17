mod student_manage_sys;

pub use student_manage_sys::{Student,Course,Class,Community,StudentManagementSystem};

fn main() {
    let mut sms: StudentManagementSystem = StudentManagementSystem::new();

    let class_a: Class = Class { name: "一年级01班".to_string(), students: vec![1, 2, 3] };
    let class_b: Class = Class { name: "一年级02班".to_string(), students: vec![4, 5, 6] };

    let course_math: Course = Course { name: "Math".to_string(), students: vec![] };//1, 3, 6
    let course_english: Course = Course { name: "English".to_string(), students: vec![] };//2, 4, 5

    let community_sport: Community = Community("Football Club".to_string(),vec![]);//1, 3, 5
    let community_computer: Community = Community("Rust Club".to_string(),vec![]);//2, 4, 6

    let student1: Student = Student::new(1, "小明".to_string(), 8, class_a.clone());
    let student2: Student = Student {id: 2,name: "小李".to_string(),..student1.clone()};
    let student3: Student = Student {id: 3,name: "小花".to_string(),..student1.clone()};

    let student4: Student = Student {id: 4,name: "张三".to_string(),class: class_b.clone(),..student1.clone()};
    let student5: Student = Student {id: 5,name: "李四".to_string(),..student4.clone()};
    let student6: Student = Student {id: 6,name: "王五".to_string(),..student4.clone()};

    sms.add_student(student1);
    sms.add_student(student2);
    sms.add_student(student3);
    sms.add_student(student4);
    sms.add_student(student5);
    sms.add_student(student6);

    sms.add_class(class_a);
    sms.add_class(class_b);

    sms.add_course(course_math.clone());
    sms.add_course(course_english.clone());

    sms.add_community(community_sport.clone());
    sms.add_community(community_computer.clone());

    //1,3,6号学生选择了数学,2,4,5，选择英语
    sms.select_course(1, &course_math);
    sms.select_course(3, &course_math);
    sms.select_course(6, &course_math);
    sms.select_course(2, &course_english);
    sms.select_course(4, &course_english);
    sms.select_course(5, &course_english);

    //1,3,5加入了足球社团，2,4,6加入计算机社团
    sms.join_community(1,&community_sport);
    sms.join_community(3,&community_sport);
    sms.join_community(5,&community_sport);
    sms.join_community(2, &community_computer);
    sms.join_community(4, &community_computer);
    sms.join_community(6, &community_computer);

    println!("Students: {:?}", sms.students);
    println!("Classes: {:?}", sms.classes);
    println!("Courses: {:?}", sms.courses);
    println!("Communities: {:?}", sms.communities);

    // sms.delete_student(2);
    // sms.delete_class("一年级02班");
    // sms.delete_course("Math");

    // println!("After deletion:");
    // println!("Students: {:?}", sms.students);
    // println!("Classes: {:?}", sms.classes);
    // println!("Courses: {:?}", sms.courses);
}
