use std::io;

fn main() {
    println!("구구단 N단 출력?");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("입력한 값을 읽지 못하였다.");

    let number: i32 = input.trim().parse()
        .expect("숫자로 변환할 수 없는 값을 입력했습니다.");

    for i in 1..=9 {
        println!("{} x {} = {}", number, i, number * i);
    }
}
