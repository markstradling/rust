fn main() {
    let number: i32 = 3;
    let number: i32 = number * 50;
    println!("{number}");
    println!("this is more text");
    println!("this is more of the same");
    let list: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for number in list{
        print!("{number}\t");
    }
    println!("");
    print!("more things");
    println!("\tthis is the next line");

}
