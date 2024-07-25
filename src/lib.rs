use rusqlite::{Connection, Result};
use std::io::{self, Write}; 
use derivative::Derivative;  

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[derive(Derivative)]
#[derivative(Debug, Default)] 
struct Todo {
	id: i32, 
	content: String, 
	priority: i32, 
	#[derivative(Default(value = "false"))]
	done: bool, 
}

fn create_table(db: &Connection) -> Result<usize>{
	db.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (), 
    )
}

fn insert(db: &Connection) {
	print!("-> Enter name: ");
	let _ = io::stdout().flush(); 
	
	let mut name = String::new(); 

	io::stdin() 
		.read_line(&mut name)
		.expect("failed to read line"); 

	let person = Person {
		id: 0, 
		name: name.trim().to_string(),
		data: None,
	};

	
	if let Err(e) = db.execute(
		"INSERT INTO person (name, data) Values (?1, ?2)", 	
		(&person.name, &person.data),
	) { 
		println!("failed to add {}: {}", &person.name, e);
	} else {
		println!("added {}", &person.name); 
	}

}

fn delete(db: &Connection) -> Result<usize> {
	print!("-> Enter id(s): "); 
	let _ = io::stdout().flush(); 

	let mut ids = String::new(); 
	
	io::stdin() 
		.read_line(&mut ids)
		.expect("failed to read line"); 

	for id in ids.split_whitespace() { 
		if let Err(e) = db.execute( 
			"DELETE FROM person WHERE ID = ?1",
			[id]
		) {
			println!("failed to delete id {}: {}", id, e); 
		} else {
			println!("deleted id {}", id);
		}
	}

	Ok(0)
}

fn display(db: &Connection) -> Result<usize> {
	let mut stmt = db.prepare("SELECT id, name, data FROM person")?;
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

	Ok(0)
}

fn help(){
	print!(
"Commands: 
    add  - add a new element to the list 
    del  - delete element(s) from the list
    list - display the list 
    help - print usage
    quit - quit mytodo 
"
	)
}


pub fn run(db: Connection) -> Result<()> {	

	let _ =	create_table(&db)?; 

	'test: loop {
		print!(">>> "); 
		let _ = io::stdout().flush();		

		let mut command = String::new();

		command.clear();

		io::stdin()
			.read_line(&mut command)
			.expect("failed to read line");

		let mut command = command.split_whitespace();

		let command = command 
			.next()
			.unwrap_or("");

		match &command[..] {
			"quit" => break 'test, 
			"add" => {
				let _ = insert(&db); 
			} 
			"del" => {
				let _ = delete(&db)?;
			} 
			"list" => { 
				let _ = display(&db)?;
			}
			"help" => help(),
			_ => { 
				println!("{:?} is not a valid command", &command);
				help(); 
			}
		}
	}

    Ok(())
}
