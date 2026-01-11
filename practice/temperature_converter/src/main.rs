use std::io;

fn main() {
    println!("[1] Convert Fahrenheit to Celsius");
    println!("[2] Convert Celsius to Fahrenheit");

    // 1. 메뉴 선택 받기 (u32 타입)
    let choice: u32 = loop {
        let input = get_input("Input the choice.");
        match input.trim().parse() {
            Ok(num) if num == 1 || num == 2 => break num,
            _ => println!("Please input number 1 or 2"),
        }
    };

    // 2. 온도 값 받기 (f64 타입)
    let temperature: f64 = loop {
        let input = get_input("Input the temperature to convert (without unit symbol).");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a valid number."),
        }
    };

    // 3. 변환 계산
    let result = if choice == 1 {
        fahrenheit_to_celsius(temperature)
    } else {
        celsius_to_fahrenheit(temperature)
    };

    if result == result.floor() { // 소수점을 내림한 값이 원래값과 같은지 확인하여 정수 판별
    println!("Result: {}", result);
    } else {
        println!("Result: {:.2}", result); // 형식 지정자 사용 - {변수명 : 상세 설정}
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

// F to C
fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.) / 1.8
}

// C to F
fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.
}
