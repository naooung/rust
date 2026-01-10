fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len(); - 타입 변경 불가에 따른 컴파일 오류 발생

    println!("The value of spaces is {}", spaces);
}