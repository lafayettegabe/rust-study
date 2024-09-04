#[cfg(test)]
mod tests {

    #[macro_export]
    macro_rules! ex_expr_macro {
        ($x: expr) => {
            format!("You sent an expression: {}", $x)
        };
    }

    #[macro_export]
    macro_rules! ex_type_macro {
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        };
    }

    #[test]
    fn tests_decl_macros() {
        let some_var: String = ex_expr_macro!(1 + 2);
        dbg!(some_var);

        let some_var: String = ex_type_macro!(i32);
        dbg!(some_var);

        let some_var: String = ex_type_macro!(u8);
        dbg!(some_var);
    }

    #[macro_export]
    macro_rules! my_vec {
        ( $($x: expr),+) => {
          {
            let mut temp_vec = Vec::new();
            $(
              temp_vec.push($x);
            )+
            temp_vec
          }
        };
    }

    #[test]
    fn tests_decl_macros_vec() {
        let x: Vec<i32> = vec![1, 2, 3];
        dbg!(x);

        let y: Vec<i32> = my_vec!(3, 2, 1);
        dbg!(y);
    }
}
