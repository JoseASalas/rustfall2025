//everywhere use genetics and traits, no if or match statements
//program to behavior only


// define two structs, undergrad and grad student
//grads student should have a thesis component
// gpa and major will be shared

struct Undergrad {
    name: String,
    gpa: f64,
    major: String,
}

struct Grad {
    name: String,
    gpa: f64,
    major: String,
    thesis: String,
}

//define trait, show_info
trait ShowInfo {
    fn show_info(&self) -> String;
}

impl ShowInfo for Undergrad {
    fn show_info(&self) -> String {
        format!("Undergrad: {}, GPA: {}, Major: {}", self.name, self.gpa, self.major)
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) -> String {
        format!("Grad: {}, GPA: {}, Major: {}, Thesis: {}", self.name, self.gpa, self.major, self.thesis)
    }
}
//create another struct called enrollment
// it should store undergrad and grad students together
// impliment show_info for all enrolled students
struct Enrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

fn enroll_student(enrollment: &mut Enrollment, student: Box<dyn ShowInfo>) {
    enrollment.students.push(student);
}

fn show_all_students(enrollment: &Enrollment) {
    for student in &enrollment.students {
        println!("{}", student.show_info());
    }
}

fn student_enrollment() {
    // create the enrollment collection
    let mut enrollment = Enrollment { students: Vec::new() };

    enroll_student(
        &mut enrollment,
        Box::new(Undergrad {
            name: "Alice".to_string(),
            gpa: 3.5,
            major: "Computer Science".to_string(),
        }),
    );

    enroll_student(
        &mut enrollment,
        Box::new(Grad {
            name: "Bob".to_string(),
            gpa: 3.8,
            major: "Mathematics".to_string(),
            thesis: "Algebraic Topology".to_string(),
        }),
    );

    show_all_students(&enrollment);
}

fn main() {
    println!("Hello, world!");
    student_enrollment();
}
