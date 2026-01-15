// [Privacy] 기본적으로 모든 아이템은 private(비공개) 상태
// 외부(main 등)에서 호출할 수 있도록 pub 키워드로 공개 설정
pub fn add_to_waitlist() {
    println!("대기 명단에 추가되었습니다.");
}