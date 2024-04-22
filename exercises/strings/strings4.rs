// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    let s1 = String::from("blue");
    let s2 = ("red".to_string());
    let s3 = (String::from("hi"));
    let s4 = ("rust is fun!".to_owned());
    let s5:String = ("nice weather".into());
    let s6 = (format!("Interpolation {}", "Station"));
    let s7 = (&String::from("abc")[0..1]);
    let s8 = ("  hello there ".trim());
    let s9 = ("Happy Monday!".to_string().replace("Mon", "Tues"));
    let s = ("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
