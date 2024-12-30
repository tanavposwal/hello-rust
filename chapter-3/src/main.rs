fn variables() {
    // only let cant be changed
    // mut with let can make that variable mutable
    // mut cant be used with const
    // const can only has capital letters
    let mut age: i32 = 18;
    println!("your current age is {age}");

    const PI: f32 = 3.14;
    println!("value of pi is {PI}");

    age += 1;
    println!("your current age is {age}");

    // shadowing
    let apples: i32 = 18;
    println!("apples {apples}");
    // upper declarations are now shadowed
    let apples: i32 = apples - 2;
    {
        let apples: i32 = apples + 20;
        println!("apples {apples}");
    }

    println!("apples {apples}");
}

fn data_types() {
    // int float char bool are only four data types present
    // i -> signed
    // u -> unsigned
    
    let num: usize = 3241;
    let num2 = 32i32; // you can define types as suffix
    let num3 = 10_00_00_000; // also valid for representing large readable numbers
    let ascii: u8 = b'A';
    
    let pi: f64 = 3.14; // floating points
    
    let yes: bool = true;

    let x: char = '𝕏';

    let tup: (i64, f64, u8, char) = (500, 6.4, 1, 'a');
    let (x, y, z, w) = tup;
    let tupchar = tup.3;

    let arr = [1,2,3,4,5,6]; // of fixed length
}

fn main() {
    // warnign there will be lots to unused variable errors
    let return_value = my_function();
    println!("return is {return_value}");
}

// function declaration
fn my_function() -> i32 {
    32
}
