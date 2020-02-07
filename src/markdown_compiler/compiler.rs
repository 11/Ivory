// Following this compiler series: https://www.youtube.com/watch?v=Eythq9848Fg

const H1: &str = &"#";
const H2: &str = &"##";
const H3: &str = &"###";
const H4: &str = &"####";
const H5: &str = &"#####";
const H6: &str = &"######";


#[derive(Debug)]
struct Token <'a> {
  group: Option<&'a str>,
  value: Option<&'a str>,
}

impl <'a> Token <'a>{
  fn new(group: Option<&'a str>, value: Option<&'a str>) -> Token <'a> {
    return Token { group, value };
  }
}



#[derive(Debug)]
struct Lexer {
  text: String,
  position: i32,
}

impl Lexer {
  fn new(text: String, position: i32, current_char: Option<char>) -> Lexer {
    return Lexer {text, position };
  }

  fn parse() {

  }

}

