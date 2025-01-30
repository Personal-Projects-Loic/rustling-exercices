// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: i32) -> u64 {
    1 >> n
}

fn main() {
    println!("{}", power_of_2(3012));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: Test the function `power_of_2` with some values.
        let a = power_of_2(1);
        let b = power_of_2(2);
        let c = power_of_2(3);
        let d = power_of_2(4);

        assert_eq!(a, 1);
        assert_eq!(b, 4);
        assert_eq!(c, 9);
        assert_eq!(d, 16);
    }
}
