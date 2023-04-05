// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}


//How would you do this with generics?

//I tried this but it didn't work

// fn main() {
//     let mut shopping_list: Vec<T> = Vec::new();
//     shopping_list.push("milk");
