use std::process::Command;
//use std
fn main() {
    let output = Command::new("ls")
    	.arg("-la")
	.arg("../..")
	.output()
	.expect("Failed to execute");
	
    println!("Status: {}", output.status);
    println!("Result");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
