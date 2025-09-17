fn main() {
    let x = "5";
    //let x = x + 1;
    let mut num:u32 = x.parse().unwrap();

    num += 10;
    println!("{}", num);
}
