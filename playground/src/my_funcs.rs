/// Function: add_five
///
/// # Arguments (num: u32)
///
/// # Returns u32
///
/// # Example
/// ```
/// let x = 5;
/// let y = add_five(x);
/// ```
///
pub fn add_five(num: u32) -> u32 {
    num + 5
}

pub fn add_ten(num: u32) -> u32 {
    num + 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x = 100;
        let y = add_five(x);
        assert_eq!(y, 105);
    }

    #[test]
    fn add_ten_test() {
        let x = 100;
        let y = add_ten(x);
        assert_eq!(y, 110);
    }
}
