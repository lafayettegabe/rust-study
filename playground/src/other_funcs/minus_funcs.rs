pub fn subtract_ten(num: u32) -> u32 {
    num - 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_five_test() {
        let x = 100;
        let y = subtract_ten(x);
        assert_eq!(y, 90);
    }
}
