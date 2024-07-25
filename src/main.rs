use rusqlite::{Connection, Result};
use std::io::{self, Write}; 

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}


fn main() -> Result<()> {
    let conn = Connection::open("./test.db")?;
  
	conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), // empty list of parameters.
    )?;

	'test: loop {
		print!(">>> "); 
		let _ = io::stdout().flush();		

		let mut buffer = String::new();

		buffer.clear();

		io::stdin()
			.read_line(&mut buffer)
			.expect("failed to read line");

		let mut buffer = buffer.split_whitespace();

		let command = buffer 
			.next()
			.unwrap_or("");

		match &command[..] {
			"quit" => break 'test, 
			"add" => {
				let name = buffer
					.next()
					.unwrap_or(""); 

				let person = Person {
					id: 0, 
					name: name.to_string(),
					data: None,
				};
			
				conn.execute(
					"INSERT INTO person (name, data) Values (?1, ?2)", 	
					(&person.name, &person.data),
				)?; 
			} 
			"del" => {
				let id = buffer 
					.next()
					.unwrap_or(""); 

				conn.execute( 
					"DELETE FROM person WHERE ID = ?1",
					[id]
				)?;
			} 
			"list" => { 
				let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
				let person_iter = stmt.query_map([], |row| {
					Ok(Person {
						id: row.get(0)?,
						name: row.get(1)?,
						data: row.get(2)?,
					})
				})?;

				for person in person_iter {
					println!("Found person {:?}", person.unwrap());
				}

			}
			_ => print!("{:?}", &buffer),
		}
	}

    Ok(())
}
