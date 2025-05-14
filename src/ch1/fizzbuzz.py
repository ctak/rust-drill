# 파이썬으로 FizzBuzz 문제를 해결하는 코드입니다.
for row in range(20):
    for col in range(5):
        number = row + col * 20 + 1
        if number % 3 == 0 and number % 5 == 0:
            text = 'FizzBuzz'
        elif number % 3 == 0:
            text = 'Fizz'
        elif number % 5 == 0:
            text = 'Buzz'
        else:
            text = str(number)

        print(f"{text:<12}", end="")
    print()
