fn example_0() {
    let r; // 'a

    {
        let x = 5; // 'b
        r = &x; // Error: `x` does not live long enough
    } // 'b dies

    println!("r: {}", r);
} // 'a dies

fn example_1() {
    // allocate space in memory
    let highest_age: i32;

    let alice_age = 20;
    let bob_age = 25;

    if alice_age > bob_age {
        highest_age = alice_age;
    } else {
        highest_age = bob_age;
    }

    println!("Highest age is: {}", highest_age);
}

fn example_2() {
    let highest_age: i32;

    let alice_age = 20;
    let bob_age = 25;

    highest_age = largest_age(&alice_age, &bob_age);

    println!("Highest age is: {}", highest_age);

    fn largest_age(alice_age: &i32, bob_age: &i32) -> i32 {
        if alice_age > bob_age {
            *alice_age
        } else {
            *bob_age
        }
    }
}

fn example_3() {
    let highest_age: &i32;

    let alice_age = 20; // 'a
    let bob_age = 25; // 'b = 'a

    highest_age = largest_age(&alice_age, &bob_age);

    println!("Highest age is: {}", highest_age);

    // 'a = 'b
    fn largest_age<'a>(alice_age: &'a i32, bob_age: &'a i32) -> &'a i32 {
        if alice_age > bob_age {
            alice_age
        } else {
            bob_age
        }
    }
} // 'a and 'b die

fn example_4() {
    let highest_age: &i32;
    let new_highest_age: i32;

    let alice_age = 20; // 'a
    {
        let bob_age = 25; // 'b

        highest_age = largest_age(&alice_age, &bob_age);
        new_highest_age = *highest_age;
    } // 'b dies

    println!("Highest age is: {}", new_highest_age);

    // 'a != 'b => b is shorter than 'a so the shortest lifetime is used
    fn largest_age<'a>(alice_age: &'a i32, bob_age: &'a i32) -> &'a i32 {
        if alice_age > bob_age {
            alice_age
        } else {
            bob_age
        }
    }
} // 'a dies

fn example_5() {
    let highest_age: &i32;
    let new_highest_age: i32;

    let alice_age = 20; // 'a
    {
        let bob_age = 25; // 'b

        highest_age = largest_age::<i32>(&alice_age, &bob_age);
        new_highest_age = *highest_age;
    } // 'b dies

    println!("Highest age is: {}", new_highest_age);

    // 'a != 'b => b is shorter than 'a so the shortest lifetime is used
    fn largest_age<'a, T: PartialOrd>(alice_age: &'a T, bob_age: &'a T) -> &'a T {
        if alice_age > bob_age {
            alice_age
        } else {
            bob_age
        }
    }
} // 'a dies

struct Person<'p> {
    name: &'p str,
    age: &'p i32, // can be diff like: age: &'q i32
}

fn example_6() {
    let highest_age: &i32;
    let new_highest_age: i32;

    let alice: Person = Person {
        name: "alice",
        age: &20,
    };

    {
        let bob: Person = Person {
            name: "bob",
            age: &25,
        };

        highest_age = largest_age::<i32>(alice.age, bob.age);
        new_highest_age = *highest_age;
    } // 'b dies

    println!("Highest age is: {}", new_highest_age);

    // 'a != 'b => b is shorter than 'a so the shortest lifetime is used
    fn largest_age<'a, T: PartialOrd>(alice_age: &'a T, bob_age: &'a T) -> &'a T {
        if alice_age > bob_age {
            alice_age
        } else {
            bob_age
        }
    }
} // 'a dies
