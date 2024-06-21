use fortune_teller::Fortune;

fn main() {
    let fortune = Fortune {
        message: "운이 좋습니다.".to_string(),
        name: "수지".to_string()
    };

    println!("{}", fortune.new_message());
}