use super::command::Command;

#[derive(Debug)]
pub struct Help<'a> {
  pub command: &'a str,
  pub args: Vec<&'a str>,
}

impl <'a> Help<'a> {
  pub fn new(command: &'a str, args: Vec<&'a str>) -> Help<'a> {
    return Help { command, args };
  }
}

impl Command for Help <'_> {
  fn validate(&self) -> bool {
    if self.args.len() != 0 {
      return false;
    }

    return true;
  }

  fn run(&self) {
    let commands = concat!(
      "oxy new [PATH] -- takes a path to where a new projcet should be built\n",
      "oxy help  -- display help screen\n",
      "oxy init  -- creates project in current folder\n",
      "oxy build -- generates project for production environment\n",
      "oxy serve -- start local server, and open project from last build\n",
      "oxy run   -- runs `oxy build` & `oxy serve`\n",
      "oxy      -- random fun message\n",
    );

    println!("{}", commands);
  }

}