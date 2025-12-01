use std::{thread, time::Duration};

fn track_changes() { //task 2
    let mut tracker = 0;
    let mut update = || {
        tracker += 5;
        println!("Tracker updated: {}", tracker);
    };

    update();
    update();
}

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32> // task 3
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

struct ComputeCache<T> // task 4
where
    T: Fn() -> String,
{
    computation: T,
    cached_result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            cached_result: None,
        }
    }

    fn get_result(&mut self) -> String {
        if let Some(ref result) = self.cached_result {
            return result.clone();
        }

        let result = (self.computation)();
        self.cached_result = Some(result.clone());
        result
    }
}

fn main() {

    let operation = |a: i32, b: i32| { //task 1
        let multiply = |x: i32, y: i32| x * y;
        let restult = multiply(a, b);
        return restult;
    };

    println!("Result: {}", operation(10, 5));
    track_changes(); // task 2

    let numbers = vec![1, 2, 3]; //task 3

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x % 2 == 0 {
            0
        } 
        else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);

    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}