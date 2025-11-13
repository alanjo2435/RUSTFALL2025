pub trait ShowInfo {
    fn show_info(&self) -> String;
}

pub struct Undergrad {
    pub major: String,
    pub gpa: f32,
}

pub struct Grad {
    pub major: String,
    pub gpa: f32,
    pub thesis: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) -> String {
        format!("Undergrad - Major: {}, GPA: {}", self.major, self.gpa)
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) -> String {
        format!(
            "Grad - Major: {}, GPA: {}, Thesis: {}",
            self.major, self.gpa, self.thesis
        )
    }
}

pub struct Enrollment<T: ShowInfo> {
    students: Vec<Box<T>>,
}

impl<T: ShowInfo> Enrollment<T> {
    pub fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    pub fn add(&mut self, student: T) {
        self.students.push(Box::new(student));
    }

    pub fn show_all(&self) {
        for s in &self.students {
            println!("{}", s.show_info());
        }
    }
}

fn main() {
    let mut undergrad_group = Enrollment::<Undergrad>::new();
    let mut grad_group = Enrollment::<Grad>::new();

    undergrad_group.add(Undergrad {
        major: "Computer Engineering".to_string(),
        gpa: 3.8,
    });

    grad_group.add(Grad {
        major: "Electrical Engineering".to_string(),
        gpa: 3.9,
        thesis: "Smart Soil and Temperature Monitoring System".to_string(),
    });

    undergrad_group.show_all();
    grad_group.show_all();
}
