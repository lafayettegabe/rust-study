/*
fn make_string_dangle() -> &String {
    let s: String = String::from("dangle");
    let r: &String = &s;
    r
}
*/

fn make_string_not_dangle() -> String {
    let s: String = String::from("dangle");
    s
}

fn main() {
    // works
    let x: i32 = 50;
    let y: i32 = x;
    println!("{}", x);
    println!("{}", y);

    // will not work
    // let s: String = String::from("hello");
    // let t: String = s;
    // println!("{}", s);

    // works
    let s: String = String::from("hello");
    let _t: String = s.clone();
    println!("{}", s);

    // works
    let s: String = String::from("hello2");
    let _t: &String = &s;
    let _u: &String = &s;
    let _v: &String = &s;
    println!("{}", s);

    // does not work
    // let _r: &String = make_string_dangle();

    let r: String = make_string_not_dangle();
    println!("{}", r);
}
