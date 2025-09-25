fn concat_strings(s1: &String, s2: &String) -> String {
    let mut comb: String = s1.to_string();
    comb.push_str(s2);
    return comb;
}

fn clone_and_modify(s: &String) -> String {
    let mut clone: String = s.to_string();
    clone.push_str("World!");
    return clone;
}

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    let mut count = low;
    while count < (high/2){
        *total += 1;
        count += 1;
    }
}

fn main() {
    // Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    // Problem 2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    //Problem 3
    let low = 0;
    let high = 100;
    let mut total = 0;
    sum(&mut total, low, high);
    println!("{}", total);
}
