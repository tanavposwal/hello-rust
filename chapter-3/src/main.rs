fn main() {
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
