# 소수인지 판단하는 함수
def is_prime(n):
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

# count만큼 소수를 생성
def get_primes(count):
    res = []
    i = 2
    while len(res) < count:
        if is_prime(i):
            res.append(i)
        i += 1
    return res

# 생성한 소수를 출력
def print_primes(primes):
    for i, prime in enumerate(primes):
        print(prime, end=", ")
        if (i + 1) % 10 == 0:
            print()  # 줄바꿈

print_primes(get_primes(100))

"""
# 이것은 코파일럿과 함게 한 것임
#
n = 1
primes = []
while True:
    n += 1
    if len(primes) >= 100:
        break
    is_prime = True
    for i in primes:
        if n % i == 0:
            is_prime = False
            break
    if is_prime:
        primes.append(n)
        print(n, end=", ")
        if len(primes) % 10 == 0:
            print()

print("\nTotal primes found:", len(primes))
"""