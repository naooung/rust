use std::io;

fn main() {
    /* 1. 스칼라 타입 (Scalar Types)
       정수형: 기본 i32 
       실수형: 기본 f64
    */
    println!("Scalar Types Section");

    let integer: i32 = -42;
    let unsigned_int: u32 = 42;
    let arch_int: usize = 1000; 

    let float64 = 2.0; 
    let float32: f32 = 3.0;

    let is_rust_fun: bool = true;

    let light_emoji = '💡'; 
    println!("Emoji: {light_emoji}");


    /* 2. 복합 타입 (Compound Types)
       튜플: 여러 타입 혼합 가능, 마침표(.)로 접근, 관련있는 데이터를 하나로 묶을 때 사용
       배열: 동일한 타입, 대괄호[]로 접근, 데이터 리스트를 다룰 때 사용
       + 두 종류 모두 선언 후 변경 불가
    */
    println!("\nCompound Types Section");

    let person: (&str, i32, bool) = ("Alice", 30, true);
    let (name, age, is_member) = person;
    let direct_name = person.0;

    println!("{direct_name} is {age} years old.");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}