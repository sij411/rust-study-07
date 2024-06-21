pub struct Fortune {
    pub message: String,
    pub name: String
}

impl Fortune {
    pub fn new_message(&self) -> String {
        
        format!("오늘 {} 님의 운세는 {} 완전 럭키비키네요!", self.name, self.message)
    }
}

pub fn main() {
    let fortune = Fortune {
        message: "대길입니다.".to_string(),
        name: "김철수".to_string()
    };

    println!("{}", fortune.new_message());
}