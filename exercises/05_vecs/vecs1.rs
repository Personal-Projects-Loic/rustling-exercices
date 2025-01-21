// TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
// Use the vector macro.

fn array_and_vec() -> (Vec<i32>, Vec<i32>) {
    let a = [40, 10, 60, 50]; // Array
    let mut v: Vec<i32> = Vec::new();

    for &value in a.iter() {
        v.push(value);
    }

    let mut b = v.clone();
    b.sort();
    (b, v)
}

fn main() {
    println!("{:?}", array_and_vec());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
