use std::io;
use std::io::stdin;

fn fahrenheit_to_celsius(f: &f64) -> f64{ // for assignment 1
    let cel = (f - 32.0) / (1.8);
    return cel;
}

fn celsius_to_fahrenheit(c: &f64) -> f64{ //For Assignment 1
    let far = (c * (1.8)) + 32.0;
    return far;
}

fn is_even(n: i32) -> bool{ //for Assignment 2
    if n % 2 == 0{
        return true;
    }
    return false;
}
fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else{
        return -1;
    }
}


fn assignment_1(){
    //variables
    let freezing = 32.0;
    let temp: &f64 = &96.00;
    print!("{}", fahrenheit_to_celsius(temp));
    println!(" ");
    let mut temp_conv = freezing;

    //loop
    while temp_conv < (freezing + 5.0){
        println!("{}", fahrenheit_to_celsius(&(temp_conv + 1.0)));
        temp_conv += 1.0;
    }
    println!(" ");
}
fn assignment_2(){
    //Array and variables
    let nums = [40, 92, 34, 31, 97, 90, 29, 24, 47, 9];
    let mut count = 0;
    let mut sum = 0;
    let mut largest = nums[0];

    //For loop for part 3
    for n in 0..nums.len(){
        if nums[n] % 15 == 0{
            println!("FizzBuzz");
        }
        else if nums[n] % 5 == 0{
            println!("Buzz");
        }
        else if nums[n] % 3 == 0{
            println!("Fizz");
        }
        else{
            println!("{}", is_even(nums[n]));
        }
    }

    //While Loop for part 4
    while count != nums.len(){
        sum = sum + nums[count];
        count += 1;
    }
    println!("{}", sum);
    count = 1;

    //Loop for Part 5
    loop{
        if count == nums.len(){
            break;
        }
        else if nums[count] > largest{
            largest = nums[count];
        }
        count += 1;
    }
    println!("{}", largest);
    println!(" ");
}

fn assignment_3(){

    //variables
    let secret = 63;
    let mut correct = -1;
    let mut tries = 0;


    loop{
        tries += 1;
        let mut input = String::new(); //User Guess
        io::stdin().read_line(&mut input).expect("A Number");
        let guess: i32 = input.trim().parse().expect("Not an integer");
        correct = check_guess(guess, secret);
        if correct == -1{
            println!("Too Low!");
        }
        else if correct == 1{
            println!("Too High!");
        }
        else {
            break;
        }
    }

    print!("Congradulations! You got the correct answer in {}", tries);
    print!(" guesses.");
    println!(" ");
}

fn main() {
    assignment_1();
    assignment_2();
    assignment_3();
}
