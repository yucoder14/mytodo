use rusqlite::{Connection, Result};
use std::process; 

fn main() -> Result<()> {
    let conn = Connection::open("./test.db")?;
  
	if let Err(e) = mytodo::run(conn) {
		eprintln!("Application error: {e}"); 
		process::exit(1);
	} 

	Ok(())
}
