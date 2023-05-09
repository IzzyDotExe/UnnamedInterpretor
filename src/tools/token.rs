pub enum Type {
    NUM,
    SUM,
    PRODUCT,
    ID,
    TRUE,
    FALSE,
    EOF
}

pub struct Token {
    pub tpe: Type,
    pub text: String,
    pub start_pos: usize
}

impl Token {
    pub fn new(in_type: Type, in_text: String, in_start: usize) -> Self {
        Token { tpe: in_type, text: in_text, start_pos: in_start }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Type::EOF => "EOF".into(),
            Type::NUM => "NUM".into(),
            Type::SUM => "SUM".into(),
            Type::PRODUCT => "PRODUCT".into(),
            Type::ID => "ID".into(),
            Type::TRUE => "BOOLT".into(),
            Type::FALSE => "BOOLF".into(),
        }
    }
}