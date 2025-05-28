# Caesar Cipher Implementation
# ord(a) => 97 // Ordinal 즉 순서로써 a 의 유니코드 "순서" 정도로 생각하면 된다.
# chr(97) => a
#
a = 'HELLO'
def cypher(text, shift):
    result = ""
    for char in text:
        if char.isalpha():
            shift_base = ord('A') if char.isupper() else ord('a')
            result += chr((ord(char) - shift_base + shift) % 26 + shift_base)
        else:
            result += char
    return result

def encrypt(text, shift):
    # 'A'와 'Z'의 문자 코드를 취득
    code_a = ord('A')
    code_z = ord('Z')
    # 결과를 대입할 변수
    result = ""
    for ch in text:
        # 문자 코드로 변환
        code = ord(ch)
        # A-Z 사이인가?
        if code_a <= code <= code_z:
          code = (code - code_a + shift) % 26 + code_a # %26 을 쓰는 이유는 XYZ => ABC 로 보여야 하기에.
        # 문자로 변환
        result += chr(code)
    return result


def encrypt_graceful(text, shift):
  a = ord('A')
  conv = lambda n: chr((ord(n) - a + shift) % 26 + a)
  enc1 = lambda n: conv(n) if 'A' <= n <= 'Z' else n
  return ''.join([enc1(n) for n in text])


if __name__ == "__main__":
  enc = encrypt("I LOVE RUST.", 3)
  dec = encrypt(enc, -3)
  print(enc, "=>", dec)

  enc2 = encrypt_graceful("I LOVE RUST.", 3)
  dec2 = encrypt_graceful(enc2, -3)
  print(enc2, "=>", dec2)