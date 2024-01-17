// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

//fn main() {
//    let cat = ("Furry McFurson", 3.5);
//    let cat2 = cat;
//    let ((&cat,[1],name),(&cat2,[2],age)) = cat;
//
//    println!("{} is {} years old.", name, age);
//}

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name,age) = cat;

    println!("{} is {} years old.", name, age);
}
