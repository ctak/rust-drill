# 어느 가계의 계산 카운터에 500원짜리 10개, 100원짜리 3개, 50원짜리 10개 있다. 잔돈으로 3950원을 거슬러 줘야 할 경우 나올 수 있는 모든 조합을 나열하시요.

# 거스름돈
price = 3950

for i500 in range(0, 11):
  for i100 in range(0, 4):
    for i50 in range(0, 11):
      if (i500 * 500 + i100 * 100 + i50 * 50) == price:
        print(f"500원 {i500}개, 100원 {i100}개, 50원 {i50}개")
