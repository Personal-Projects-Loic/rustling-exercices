fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a = [0; 101];

    for i in 0..101 {
        a[i] = i as i32;
    }

    for i in 0..99 {
        println!("array au rang {} = {}", i, a[i]);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
