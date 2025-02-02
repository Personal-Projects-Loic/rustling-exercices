fn slice_array(n: usize) -> i32 {
    let tab = [1, 2, 3, 4, 5];

    println!("Len of the tab : {}", tab.len());

    if n <= tab.len() || n > 1 {
        let slice = &tab[1..n];
        println!("the tab before : {:?}\nSlice now : {:?}", tab, slice);
        0
    } else {
        println!("No");
        1
    }
}

fn main() {
    slice_array(3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice = &a[1..3];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
