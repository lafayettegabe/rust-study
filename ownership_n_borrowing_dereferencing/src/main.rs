fn main() {
    // Deref coercion

    let mut name: String = String::from("Gabriel");
    let name_t: &mut String = &mut name;

    *name_t = String::from("LaFayette");

    println!("{}", name_t);
    println!("{}", *name_t);
    println!("{:p}", name_t);

    println!("{}", name);

    // ------------------------

    let mut x: i32 = 50;
    x = 70;
    dbg!(x);

    let y: &mut i32 = &mut x;
    *y += 1;
    dbg!(y);
}
