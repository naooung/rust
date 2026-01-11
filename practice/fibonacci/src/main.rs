use std::io;

fn main() {
    println!("--- Fibonacci Calculator ---");
    println!("[1] Iterative (반복문 방식)");
    println!("[2] Recursive (재귀 함수 방식)");

    // 1. 실행 방식 선택 받기
    let mode : u32 = loop {
        let input = get_input("Choose the method:");
        match input.trim().parse() {
            Ok(num) if num == 1 || num == 2 => break num,
            _ => { // 와일드 카드인 _를 사용하여 나머지 모든 경우 처리
                println!("Please enter 1 or 2.");
                continue;
            }
        }
    };

    // 2. n번째 숫자 입력 받기
    let n = loop {
        let input = get_input("Enter the 'n' for n-th Fibonacci:");
        match input.trim().parse() {
            Ok(num) => break num,
            _ => println!("Please enter a valid number."),
        }
    };

    // 3. 선택에 따라 다른 함수 호출
    let result = if mode == 1 {
        fibonacci_iterative(n)
    } else {
        fibonacci_recursive(n)
    };

    println!("{}th fibonacci number is {}.", n, result);
}

// 공통 입력 함수
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fail to read line");
    input
}

// 방법 1: 반복문(Iterative) 방식
fn fibonacci_iterative(num: u32) -> u32 {
    if num <= 2 { return 1; }
    let mut n1 = 1;
    let mut n2 = 1;

    for _ in 3..=num { // 3..num+1와 동일
        let tmp = n1;
        n1 = n2;
        n2 = tmp + n1;
    }
    n2
}

// 방법 2: 재귀(Recursive) 방식
fn fibonacci_recursive(num: u32) -> u32 {
    if num <= 2 {
        return 1;
    }
    fibonacci_recursive(num - 1) + fibonacci_recursive(num - 2)
}