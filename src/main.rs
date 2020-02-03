use std::env;
use oxygen::commands;
use oxygen::commands::command::Command;

fn main() {
  let mut cmds: Vec<String> = env::args().collect();

  let mut oxy_input = cmds.split_off(2);
  println!("oxy input: {:?}", oxy_input);

  let oxy_cmd: String = oxy_input.remove(0);
  println!("oxy cmd: {:?}", oxy_cmd);

  let oxy_args = oxy_input;
  println!("oxy args: {:?}", oxy_args);

  let help_cmd = commands::help::Help::new(oxy_cmd, oxy_args);
  let should_run = help_cmd.validate();

  if should_run {
    help_cmd.run();
  }
}