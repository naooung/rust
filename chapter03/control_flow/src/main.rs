fn main() {
    /* if 표현식
       조건식은 반드시 bool 타입이어야 함 (0이나 1은 안됨)
       if는 수식이므로 let 문 오른쪽에서 변수에 값을 대입할 수 있음
    */
    println!("--- if Expressions ---");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is NOT divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5 
    } else { 
        6 
    }; // if-else의 반환 타입은 일치해야 함
    println!("The value of number is: {number}");


    /* loop 반복문
       무한 반복하며, break 뒤에 값을 적어 루프 외부로 결과 반환 가능
       중첩 루프 시 '라벨을 붙여 원하는 바깥 루프를 한 번에 종료 가능
    */
    println!("\n--- loop & Labels ---");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 값을 반환하며 종료
        }
    };
    println!("The loop result is {result}");

    'outer: loop {
        loop {
            break 'outer; // 라벨을 지정하여 바깥 루프까지 탈출
        }
    }


    /* while & for 반복문
       while: 조건이 true인 동안 반복
       for: 컬렉션(배열 등)의 요소를 안전하고 빠르게 순회 (가장 권장됨)
    */
    println!("\n--- while & for ---");
    let mut count = 3;

    while count != 0 {
        println!("{count}!");
        count -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    // .rev를 사용한 범위 반전 역순 순회
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}