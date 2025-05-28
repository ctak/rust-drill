fn main() {
  // 거스름돈
  let price: i64 = 3950;
  // 각 동전이 몇 개 있는지 정의
  let count500: i64 = 10;
  let count100: i64 = 3;
  let count50: i64 = 10;
  // 반복문을 통해 거스름돈 조합 계산
  for i500 in 0..(count500 + 1) {
    for i100 in 0..(count100 + 1) {
      for i50 in 0..(count50 + 1) {
        // 현재 조합의 총액 계산
        let total: i64 = i500 * 500 + i100 * 100 + i50 * 50;
        // 총액이 거스름돈과 일치하는지 확인
        if total == price {
          println!("500원 {}개, 100원 {}개, 50원 {}개", i500, i100, i50);
        }
      }
    }
  }
}
