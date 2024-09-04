#[derive(Debug)]
enum Message {
    Quit,
    ChangeColour(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn procress_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit!");
        }
        Message::ChangeColour(red, green, blue) => {
            println!("Red {}, Green {}, Blue {}", red, green, blue);
        }
        Message::Move { x, y } => {
            println!("X is {}, Y is {}", x, y);
        }
        Message::Write(text) => {
            println!("{}", text)
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_match_literals() {
        let x = 20;

        let res = match x {
            1 => "first",
            2 | 3 | 5 | 7 | 15 | 20 => "another few",
            _ => "something else",
        };

        println!("{}", res);
    }

    #[test]
    fn tests_match_options_1() {
        let some_num: Option<i32> = Some(10);

        let res = match some_num {
            Some(i) => i,
            None => {
                panic!("There was a problem");
            }
        };

        println!("{:?}", some_num);
        println!("{}", res);
    }

    #[test]
    fn tests_match_options_2() {
        let some_num: Option<i32> = Some(10);

        if let Some(i) = some_num {
            println!("{}", i);
        } else {
            panic!("There was a problem");
        }
    }

    #[test]
    fn tests_match_options_3() {
        let some_num: Option<i32> = Some(10);

        let my_int = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };

        println!("{}", my_int);
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("There was a problem");

        let res = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        println!("{}", res);

        let my_int: i32 = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem")
        };

        println!("{}", my_int);
    }

    #[test]
    fn tests_large_enum() {
        let my_quit = Message::Quit;
        procress_message(my_quit);

        let my_colour = Message::ChangeColour(10, 20, 255);
        procress_message(my_colour);

        let my_move = Message::Move { x: 10, y: 200 };
        procress_message(my_move);

        let my_write = Message::Write("Hello World!".to_string());
        procress_message(my_write);
    }

    #[test]
    fn tests_match_guard() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("They match!"),
            (x, y) if x + y == 0 => println!("They neutralize"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("Dont care..."),
        }
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location = Location { x: 0, y: 20 };

        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("X and Y are not on the axis"),
        }
    }
}
