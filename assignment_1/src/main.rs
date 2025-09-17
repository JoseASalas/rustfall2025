fn fahrenheit_to_celsius(f: f64) -> f64{
    let far = f;
    let mut cel = (far - freezing) * (5/9);
    return (cel);
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    let celc = c;
    let mut fare = (celc * (9/5)) + freezing;
    return (fare);
}

fn assignment_1(){
    let freezing = 32.0;
    let mut temp = 96;
    print(fahrenheit_to_celsius(temp));
    let mut temp_conv = freezing;
    while temp_conv < freezing + 5{
        print(fahrenheit_to_celsius(temp_conv + 1));
        temp_conv += 1;
    }
}

fn main() {
    assignment_1();
}
