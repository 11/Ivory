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
      "oxy new [PATH] -- Takes a path to where a new projcet should be built\n",
      "oxy init  -- Creates project in current folder\n",
      "oxy build -- Generates project for production environment\n",
      "oxy serve -- Start local server and open project from last build\n",
      "oxy run   -- Runs `oxy build` & `oxy serve`\n",
    );

    println!("{}", commands);
  }

}