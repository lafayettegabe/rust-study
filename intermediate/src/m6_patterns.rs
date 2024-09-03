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
}
