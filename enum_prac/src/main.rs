#[derive(Debug)]
enum Fruit{
    Apple(String),
    Banana(String), 
    Tomato(String),
}

struct Inventory{
    fruit:Vec<Fruit>,
}
impl Inventory{
    fn available_fruits(&self) {
        for f in &self.fruit{
           println!("{:?}", f);
           Self::tell_me_joke(f);
        }
    }
    
    fn tell_me_joke(fruit:&Fruit){
        match fruit{
            Fruit::Apple(msg) => println!("{:?}", msg),
            Fruit::Banana(msg) => println!("{:?}", msg),
            Fruit::Tomato(msg) => println!("{:?}", msg),
        }
    }
}
fn main() {
    let a = "Appealing to an apple is hard.".to_string();
    let b = "Bananas help you give others the slip.".to_string();
    let t = "All right, give me the sause.".to_string();
    let fruits = vec![Fruit::Banana(b),Fruit::Apple(a),Fruit::Tomato(t)];
   //bad example let fruit = 1; //Banana
   
   let grocery_store = Inventory{
    fruit:fruits,
   };
   
   grocery_store.available_fruits();
}
