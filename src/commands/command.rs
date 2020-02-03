use std::env;

use super::help::Help;


/// # Commands
/// - `oxy new [PATH]`: takes a path to where a new projcet should be built
/// - `oxy init`: creates project in current folder
/// - `oxy build`: generates project for production environment
/// - `oxy serve`: start local server, and open project from last build
/// - `oxy run`: runs `oxy build` & `oxy serve`
/// - `oxy help`: display help screen
/// - `oxy`: random fun message
pub trait Command {
  fn validate(&self) -> bool;
  fn run(&self);
}


pub fn parse_command() {
  // command line arguments
  let mut cmds: Vec<String> = env::args().collect();

  // remove binary file name and `oxy` keyword from input
  let mut oxy_input = cmds.split_off(2);

  // (like new, init, etc.) and arguments
  let oxy_cmd: String = oxy_input.remove(0);
  let oxy_args: Vec<String> = oxy_input;

  let cmd = match oxy_cmd.as_str() {
    "new"   => None,
    "init"  => None,
    "build" => None,
    "serve" => None,
    "run"   => None,
    "help"  => Some(Box::new(Help::new(oxy_cmd, oxy_args))),
    _       => None,
  };

  if let Some(Command) = cmd {
    let is_valid_cmd = cmd.unwrap().validate();

    if is_valid_cmd {
      cmd.unwrap().run();
    }
  }

}