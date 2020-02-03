use super::command::Command;

#[derive(Debug)]
pub struct Help {
  pub command: String,
  pub args: Vec<String>,
}

impl Help {}

impl Command for Help {

  fn new(command: String, args: Vec<String>) -> Help{
    return Help { command, args };
  }

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