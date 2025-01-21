fn main() {
    let mut x = Vec::new();

    x.push(42);
    x.push(13);

    let y = &mut x;
    y.push(42);
    let z = &mut x;
    z.push(13);

    x.sort();

    println!("{:?}", x);
}
