fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to pop elements from the vector
    if let Some(last) = vec.pop() {
        println!("Last element: {:?}", last);
    } else {
        println!("Vector is empty");
    }

    if let Some(second_last) = vec.pop() {
        println!("Second to last element: {:?}", second_last);
    } else {
        println!("Vector is empty");
    }
} 