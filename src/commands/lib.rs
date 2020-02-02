/// Commands
/// oxy new [PATH] -- takes a path to where a new projcet should be built
/// oxy help       -- display help screen
/// oxy init       -- creates project in current folder
/// oxy build      -- generates project for production environment
/// oxy serve      -- start local server, and open project from last build
/// oxy run        -- runs `oxy build` & `oxy serve`
/// oxy            -- random fun message

trait Command {
  fn new(command: String, args: Vec<String>) -> Self;
  fn validate(&self) -> bool;
  fn run(&self);
}