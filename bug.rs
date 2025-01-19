fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will cause a panic because pop() on an empty vector is invalid
    let last = vec.pop();
    println!("Last element: {:?}", last);
    // This will also cause a panic because accessing the last element of an empty vector is invalid
    let second_last = vec.pop();
    println!("Second to last element: {:?}", second_last);
}