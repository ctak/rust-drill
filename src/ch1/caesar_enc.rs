// 암호화 함수
fn encrypt_copilot(text: &str, shift: i16) -> String {
    let mut encrypted_text = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
            let shifted = ((c as u8 - base + shift as u8) % 26 + base) as char;
            encrypted_text.push(shifted);
        } else {
            encrypted_text.push(c);
        }
    }
    encrypted_text
}

fn encrypt(text: &str, shift: i16) -> String {
  // 'A'와 'Z'의 문자 코드를 i16 타입으로 취득
  let code_a = 'A' as i16;
  let code_z = 'Z' as i16;
  // 암호화된 문자열을 저장할 변수
  let mut result = String::new();
  for ch in text.chars() {
    let mut code = ch as i16;
    // A부터 Z까지의 문자라면
    if code_a <= code && code <= code_z {
      code = (code - code_a + shift + 26) % 26 + code_a;
    }      // 문자 코드를 문자로 변환하여 결과에 추가
    // 문자 코드를 다시 문자로 변환
    result.push((code as u8) as char);
  }
  return result;
}

fn multiply(a: i16, b: i16) -> i16 {
  a * b // 끝이 ;으로 끝나지 않는군. js 의 => 함수란 비슷하네.
}

fn main() {
  let enc = encrypt("I LOVE RUST", 3);
  let dec = encrypt(&enc, -3);
  println!("{} => {}", enc, dec);

  // let ex1 = multiply(3, 5);
  // println!("3 * 5 = {}", ex1);
  // let enc2 = encrypt_copilot("I LOVE RUST", 3);
  // let dec2 = encrypt_copilot(&enc2, -3); // 문자열을 String &str
  // println!("{} => {}", enc2, dec2);

  // 클로저
  let x2 = |n| n*2;
  println!("{}", x2(2));
  println!("{}", x2(8)); // => 16
}