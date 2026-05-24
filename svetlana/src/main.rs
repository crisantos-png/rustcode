use std::process::Command;
use std::io;
use std::io::Write;

fn main() {

	let mut _princessa = Command::new("clear")
		.status();

	
	loop {
		print!(">> ");
		io::stdout().flush().expect("Failed to flush the toilet");
		
		let mut user = String::new();
		user.clear();
		io::stdin()
			.read_line(&mut user)
			.expect("Failed to read line");
	
		let words: Vec<&str> = user.split_whitespace().collect();

	
		
	
		if words.is_empty() {continue;}
		if words[0] == "exit" {println!("Goodbye sir"); return;}

		let program = words[0];
		let arguments = &words[1..];
	
		
	    let process = Command::new(program)
		   	.args(arguments)
			.output();
		match process {
			Ok(output) => {
				if output.status.success() {
					println!("Done");
					println!("{}", String::from_utf8_lossy(&output.stdout));
				}
				else {
					eprintln!("Command executed but failed: {}", String::from_utf8_lossy(&output.stderr));
				}
			}
			Err(e) => {
				if e.kind() == std::io::ErrorKind::NotFound {
					eprintln!("Error, command {} not found", program);
				}
				else {
					eprintln!("An error occured: {}", e);
				}
			}
		}
		
	    
	}
}
