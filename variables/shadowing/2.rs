// Remove a line in the code to make it compile

// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and rebinding
//     let x = x;
//     x += 3;

//     let y = 4;
//     // Shadowing 
//     let y = "I can also be bound to text!";

//     println!("Success!");
// }

fn main() {
    let mut x: i32 = 1;
    x = 7;
    //shadowing and rebining
    //let mut x = x;
    x += 3;

    let y: i32 = 4;
    let y = "I can also be bound to text";
    println!("Success");
}