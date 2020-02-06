// Following this compiler series: https://www.youtube.com/watch?v=Eythq9848Fg

const H1: &str = &"#";
const H2: &str = &"##";
const H3: &str = &"###";
const H4: &str = &"####";
const H5: &str = &"#####";
const H6: &str = &"######";


#[derive(Debug)]
struct Token <'a> {
  class: Option<&'a str>,
  value: Option<&'a str>,
}

impl <'a> Token <'a>{
  fn new(class: Option<&'a str>, value: Option<&'a str>) -> Token <'a> {
    return Token { class, value };
  }
}



#[derive(Debug)]
struct Lexer {
  text: String,
  position: i32,
  current_char: Option<char>,
}

impl Lexer {
  fn new(text: String, position: i32, current_char: Option<char>) -> Lexer {
    return Lexer {text, position, current_char};
  }
}