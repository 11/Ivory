use crate::Command;


#[dervie(Debug, Command)]
struct Help {
  command: String,
  args: Vec<String>,
}


impl Command for Help {

  fn new(command: String, args: Vec<String>) -> HelpCommand {
    return HelpCommand{ command, args };
  }

  fn validate(&self) -> bool {
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
      " oxy      -- random fun message\n",
    );

    println!("{}", commands);
  }

}
