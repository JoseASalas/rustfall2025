// fn essence_example_closure() {
//     let x = 21;
//     let get_answer = |y: i32| x + y;
//     println!("{:?}", get_answer(21));  // Output: 42
// }

// fn using_function_as_variable() {
//     // Regular function
//     fn add(x: i32, y: i32) -> i32 {
//         x + y
//     }

//     // Function pointer
//     let _f = add;

//     // Closure with explicit types
//     let _f = |x: i32, y: i32| { x + y };

//     // Simplified closure
//     let _f = |x: i32, y: i32| x + y;

//     // Closure with inferred types
//     let _f = |x, y| x + y;
    
//     let result = _f(1, 2);
//     println!("{}", result);  // Output: 3
// }

// fn using_function_as_parameter() {
// let multiply = |x, y| x * y;
// let x:f32 = 1.0;
// let z:f32 = 2.0;
// let divide = |y: f32| (x + z) / y;

//     // fn add(x: i32, y: i32) -> i32 {
//     //     x + y
//     // }

//     // fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
//     //     let result = operation(x, y);
//     //     println!("Result of operation: {}", result);    
//     // }

//     // calculator(1, 2, add);
//     // calculator(1, 2, |x, y| x + y);
//     // calculator(1, 2, |x, y| x - y);
//     // calculator(1, 2, |x, y| x * y);
//     // calculator(1, 2, |x, y| x / y); // Returns 0 (stuck as an int)
//     let res = multiply(10, 20);
//     println!("Multiplication Result: {}", res);  // Output: 200
//     let result = divide(2 as f32);
//     println!("Division Result: {}", result);  // Output: 5.0
// }

// fn box_pointer_intro() {
//     let box_default = Box::new(100);
//     println!("{}", box_default);  // Output: 100
// }

// fn box_polymorphism() {
//     use core::fmt::Debug;
    
//     trait Animal: Debug {
//         fn sound(&self) -> String;
//     }
    
//     #[derive(Debug)]
//     struct Dog{
//         name: String,
//     }
    
//     impl Animal for Dog {
//         fn sound(&self) -> String {
//             format!("Woof woof. My dog's name is {}", self.name)
//         }
//     }
//     // inside of the struct, crate a field called name
//     // and beside sound, your animal shoul print my name is {name}
    
//     #[derive(Debug)]
//     struct Cat{
//         name: String,
//     }
    
//     impl Animal for Cat {
//         fn sound(&self) -> String {
//             format!("Meow meow! My cat's name is {}", self.name)
//         }
//     }
    
//     let mut zoo: Vec<Box<dyn Animal>> = Vec::new();// <dyn Animal> we were achieving polymorphism by using trait bounds, but now we are using trait objects to achieve polymorphism.
    
//     zoo.push(Box::new(Dog{name: "Rover".to_string()}));
//     zoo.push(Box::new(Cat{name: "Whiskers".to_string()}));
    
//     for animal in zoo {
//         println!("{:?} {}", animal, animal.sound());
//     }
// }

fn using_closure_as_parameter_and_capture_environment() {
    
    fn add(x: i32, y:i32) -> i32 {
        x + y
    }

    // story here changes dramatically.
    // Fn is a trait, which is needed to be dispatched at the runtime
    // Box puts that function into heap
    fn calculator(operation: Box<dyn Fn() -> i32 + '_>) {
        let result = operation();
        println!("result of operation {}", result);    
    }

    // calculator(1,2,Box::new(add)); 
    // calculator(1,2,Box::new(|x,y| x + y)); // works as expected

    let z = 3;
    let x = 2;
    let y = 5;
    calculator(Box::new(|| x + y + z));  // works as expected, but z is not passed as a parameter, but it is captured by the closure and can be used inside the closure.
    // z is an unexpected guess in our closure, because it's not passed,
    // between pipes, but due to the nature of closures which captures the environment I can stil access it and need to make sure to incdicate it's lifetime.

}

fn capture_modify_environment() {
    let mut result = 0;

    // let mut calculator = |x, y| { result = x + y };
    // calculator(1, 2);
    // println!("{}", result);  // Output: 3
    
    // Using FnMut trait
    let mut calculator: Box<dyn FnMut(i32, i32)> = Box::new(|x, y| { result = x + y });
    calculator(1, 2);
    drop(calculator);
    println!("{}", result);  // Output: 3
}

fn capture_ownership_modify() {
    let nums = vec![1, 2, 3, 4, 5].into_iter();
    let name = "Closure Example".to_string();
    //println!("Name: {}", name);
    
    let product_through_iterator: Box<dyn FnOnce() -> i32> = Box::new(move || {
        println!("Name: {}", name); 
        nums.product()});
    let result: i32 = product_through_iterator();
    println!("{}", result);  // Output: 120
}

fn main() {
    println!("Hello, world!");
    // essence_example_closure();
    // using_function_as_variable();
    // using_function_as_parameter(); 
    // box_pointer_intro();   
    // box_polymorphism();
    using_closure_as_parameter_and_capture_environment();
    capture_modify_environment();
    capture_ownership_modify();
}
