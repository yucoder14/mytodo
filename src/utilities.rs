use std::io::{self, Write}; 

pub fn prompt_read_stdin(prompt: &str, buf: &mut String) {
	print!("{}", prompt); 
	let _ = io::stdout().flush(); 

	io::stdin() 
		.read_line(buf)
		.expect("failed to read line"); 
}
