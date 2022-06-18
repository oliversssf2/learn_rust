fn main() {
    let mut a = [1, 2, 3, 4, 5];
    let slice = &mut a[1..3];
    slice[0] = 9;
    // assert_eq!(slice, &[2,3]);
    println!("{:?}", slice);
    println!("{:?}", a);
}
