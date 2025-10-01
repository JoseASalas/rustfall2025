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

    fn get_name (&self) -> &String{
        return &self.name;
    }
    fn set_major(&mut self, new_major: String){
        self.major = new_major;
    }
} 

fn main() {

    let mut word = "UT".to_string();
    word.push_str("RGV");
    println!("{word}");

    let me = Student::new("Jose Salas".to_string(), "Computer Science".to_string());
    let mut friend = Student::new("Tony Bandero".to_string(), "Computer Science".to_string());
    println!("{}", me.name);
    println!("{}", me.major);
    println!("{}", friend.name);
    println!("{}", friend.get_name());
    friend.set_major("Computer Enjineering".to_string());
    println!("{}", friend.major);
}
