use std::io;
fn main() {
    let mut x = 5;
    println!("This first value of x variable is: {}", x);
    x = 6;
    println!("This after value of x variable is: {}", x);
    let value = 42;
    println!("{} is the answer. The answer is {}. Did you know that {} is the answer?", value,x,x); // Cách 1
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Giá trị của hằng số là:{}", THREE_HOURS_IN_SECONDS);

    {
        let x = x + 1;
        println!("Gia tri moi cua x la: {}", x);
    }
    println!("Gia tri ban dau cua x la: {}", x);

    let space = "      ";
    let count: usize = space.len();
    println!("So ki tu co trong bien space la: {}", count);

    let var_input = "123456";
    let var_input: u32 = var_input.parse().expect("Không phải là số");
    println!("Giá trị của biến var_input là: {}", var_input);

    println!("Kieu du lieu char:");
    let char_01 = 'V';
    println!("Gia tri bien char_01 là: {}", char_01);

    println!("Khai bao bien tuples:");
    let tuples_01: (i32,u64,f64)= (-100, 500, 12.78);
    let (x, y, z) = tuples_01;
    println!("Gia tri cua y, z, x trong tuples_01 la: {}, {}, {}", y, z, x);
    println!("Gia tri cua z trong tuples_01 la: {}", tuples_01.2);

    //Đây là phần code kiểm chứng truy cập vượt phạm vi mảng
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


}
