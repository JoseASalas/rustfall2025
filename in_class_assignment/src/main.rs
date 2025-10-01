struct Student {
    name: String,
    major: String
}

impl Student{
    fn new(n:String, m:String) -> Student{
        Student{
            name: n,
            major: m
        }
    }

    fn get_major (&self) -> &String{
        return &self.major;
    }
    fn set_major(&mut self, new_major: String){
        self.major = new_major;
    }
} 

fn main() {
    let mut me = Student::new("Jose Salas".to_string(), "Computer Science".to_string());
    println!("{}", me.name);
    println!("{}", me.major);
    me.set_major("Computer Engineering".to_string());
    let new_major = me.get_major();
    println!("{}", new_major);
}
