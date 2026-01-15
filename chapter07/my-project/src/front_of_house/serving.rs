// [Grouping] 서빙과 관련된 로직을 별도 모듈로 분리하여 관리
pub fn take_order() {
    println!("주문을 받습니다.");
}

pub fn serve_order() {
    println!("음식을 서빙합니다.");
}

pub fn take_payment() {
    println!("결제를 진행합니다.");
}