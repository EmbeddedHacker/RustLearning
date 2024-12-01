// Use mut to mark a vatiable to make it compile

// Fill the blanks in the code to make compile
// fn main() {
//     let __ __ = 1;
//     __ += 2;

//     asset_eq!(x,3);
//     println!("Success!");
// } 

fn main() {
    let mut x : i32 = 1;
    x += 2;

    assert_eq!(x,3);
    println!("Success");
}