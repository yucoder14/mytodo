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

fn prompt_read_stdin(prompt: &str, buf: &mut String) {
	print!("{}", prompt); 
	let _ = io::stdout().flush(); 

	io::stdin() 
		.read_line(buf)
		.expect("failed to read line"); 
}

fn create_table(db: &Connection, table_name: &str ) -> Result<usize>{
	db.execute(
        &format!("CREATE TABLE IF NOT EXISTS {table_name} (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )")[..],
        (), 
    )
}

fn insert(db: &Connection) {
	let _ = io::stdout().flush(); 
	
	let mut name = String::new(); 

	prompt_read_stdin( 
		"-> Enter name: ",
		&mut name,
	);

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
	let _ = io::stdout().flush(); 

	let mut ids = String::new(); 

	prompt_read_stdin( 
		"-> Enter id(s): ",
		&mut ids,
	);

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
	let mut table_name = String::new(); 

	prompt_read_stdin(
		"Please enter list to create/modify: ", 
		&mut table_name,
	);
	
	let _ =	create_table(&db, &table_name)?; 

	'mytodo: loop {
		let mut command = String::new();

		command.clear();

		prompt_read_stdin(
			&*format!("{} >>> ", &table_name.trim()), 
			&mut command,
		);
		
		let mut command = command.split_whitespace();

		let command = command 
			.next()
			.unwrap_or("");

		match &command[..] {
			"" => print!(""),
			"clear" => { 
				print!("\x1B[2J\x1B[1;1H");
			}
			"quit" => break 'mytodo, 
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
