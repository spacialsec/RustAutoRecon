use std::env;
use std::process::Command;

fn main() {
   let args: Vec<String> = env::args().collect();

   let output = Command::new("nice")
       .args(&["-n"])
       .args(&["20"]) // Make Blazing Fast
       .args(&["autorecon"])
       .args(&args[1..])
       .output()
       .expect("Failed to execute command");

   println!("{}", String::from_utf8_lossy(&output.stdout));
}
 
 
