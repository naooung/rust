use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // --- 1. panic! : 복구 불가능한 에러 ---
    // 프로그램을 즉시 종료해야 할 때 사용 (예: 설정 파일 부재)
    // panic!("치명적 오류 발생!"); 

    // --- 2. Result와 match : 세밀한 복구 로직 ---
    let file_result = File::open("hello.txt");

    let _greeting_file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일 생성 실패: {:?}", e),
            },
            other_error => panic!("파일 열기 실패: {:?}", other_error),
        },
    };

    // --- 3. unwrap & expect : 빠르고 간결한 처리 ---
    // unwrap: 성공을 확신할 때 사용. 실패 시 기본 메시지로 panic!
    let _num = "100".parse::<i32>().unwrap();

    // expect: 실패 시 내가 작성한 메시지를 출력하며 panic! (디버깅에 유리)
    let _port = "8080".parse::<u16>()
        .expect("포트 번호 형식이 잘못되었습니다.");

    // --- 4. unwrap_or_else : 우아한 플랜 B (클로저 활용) ---
    // 에러 발생 시 프로그램을 종료하지 않고 대안값을 계산함
    let user_name = fetch_user_name().unwrap_or_else(|err| {
        println!("사용자명을 가져오지 못함({:?}). 기본값 'Guest'를 사용합니다.", err);
        String::from("Guest")
    });
    println!("로그인 유저: {}", user_name);

    // --- 5. ? 연산자 : 에러 전파 (Short-cut) ---
    match read_username_from_file() {
        Ok(name) => println!("파일에서 읽은 이름: {}", name),
        Err(e) => println!("에러 전파됨: {:?}", e),
    }
}

/// 에러 전파 시뮬레이션 함수
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    // ?는 에러 발생 시 즉시 return Err(e)를 실행함
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// 임시 에러 발생 함수
fn fetch_user_name() -> Result<String, String> {
    Err(String::from("DatabaseConnectionError"))
}

// --- 6. 커스텀 타입을 이용한 유효성 검증 (Validation) ---
// 실행문(if)에서 매번 검사하는 대신, 객체 생성 시점에 검증을 강제함.

pub struct Guess {
    value: i32, // 외부에서 직접 수정 못하도록 pub을 붙이지 않음 (캡슐화)
}

impl Guess {
    /// 새로운 Guess를 생성, 1~100 사이가 아니면 panic 발생
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // 자바의 throw new IllegalArgumentException()와 유사
            panic!("범위를 벗어난 값입니다! (입력: 1~100, 실제: {})", value);
        }

        Guess { value }
    }

    /// 내부 값을 읽기 위한 전용 메서드 (Getter)
    pub fn value(&self) -> i32 {
        self.value
    }
}