const MY_INTEGER: u8 = 10;
const MSG_CONST: &str = "hello_constant";

fn main() {
    // stack
    let x: u8 = 50;
    println!("x is {}", x);

    // heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    arr.push(10);
    println!("vec is {:?}", arr);

    // a reference on the stack poiting to a value on the heap
    let arr_2 = &arr[0..3];
    println!("arr_2 is {:?}", arr_2);

    // heap
    let mut s: String = String::from("Gabriel Soares");
    s.push_str(" ");
    s.push_str("!");
    println!("s is {:?}", s);

    // a reference on the stack poiting to a value on the heap
    let s_2 = &s[0..5];
    println!("s_2 is {:?}", s_2);

    println!("MY_INTEGER is {:?}", MY_INTEGER);

    // -----------------------------------------------------------

    let s: String = String::from("Hello String");
    let s_2: &str = &s[0..5];
    println!("{}", s_2);

    let msg: &str = "hello2";
    println!("{}", msg);

    let msg_string: String = "hello3".to_string();
    println!("{}", msg_string);

    println!("{}", MSG_CONST);
}
