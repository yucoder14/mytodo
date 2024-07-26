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
        &*format!("CREATE TABLE IF NOT EXISTS {table_name} (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )"),
        (), 
    )
}

fn insert(db: &Connection, table_name: &str) {
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
		&*format!("INSERT INTO {table_name} (name, data) Values (?1, ?2)"), 	
		(&person.name, &person.data),
	) { 
		println!("failed to add {}: {}", &person.name, e);
	} else {
		println!("added {}", &person.name); 
	}
}

fn delete(db: &Connection, table_name: &str) {
	let _ = io::stdout().flush(); 

	let mut ids = String::new(); 

	prompt_read_stdin( 
		"-> Enter id(s): ",
		&mut ids,
	);

	for id in ids.split_whitespace() { 
		if let Err(e) = db.execute( 
			&*format!("DELETE FROM {table_name} WHERE ID = ?1"),
			[id]
		) {
			println!("failed to delete id {}: {}", id, e); 
		} else {
			println!("deleted id {}", id);
		}
	}
}

fn display(db: &Connection, table_name: &str) -> Result<usize> {
	let mut stmt = db.prepare(&*format!("SELECT id, name, data FROM {table_name}"))?;
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

fn help_inner(){
	print!(
"Commands: 
    add  - add a new element to the list 
    del  - delete element(s) from the list
    list - display the list 
    help - print help
    quit - quit interactive mode
"
	)
}

fn inner_loop(db: &Connection, table_name: &str) -> Result<()> {
	'inner: loop {
		let mut command = String::new();

		command.clear();

		prompt_read_stdin(
			&*format!("({}) >>> ", &table_name.trim()), 
			&mut command,
		);

		let command = command.trim();		

		match command {
			"" => print!(""),
			"clear" => print!("\x1B[2J\x1B[1;1H"),
			"quit" => break 'inner, 
			"add" => insert(db, table_name), 
			"del" => delete(db, table_name),
			"list" => { 
				let _ = display(db, table_name)?;
			}
			"help" => help_inner(),
			_ => { 
				println!("{:?} is not a valid command", &command);
				help_inner(); 
			}
		}
	}

	Ok(())
}

fn display_tables(db: &Connection) -> Result<()>{
	let mut stmt = db.prepare(
		"SELECT 
			name 
		FROM 
			sqlite_schema 
		WHERE
			type = 'table' AND 
			name NOT LIKE 'sqlite_%'",
	)?; 

	let rows = stmt.query_map([], |row| {
		let name: String = row.get(0)?; 
		Ok(name)
	})?;

	for table in rows {
		print!("{} ", table.unwrap()); 
	} 

	println!("");

	Ok(())
}

fn select_table(db: &Connection) -> Result<()> {
	let mut table_name = String::new(); 

	prompt_read_stdin(
		"-> Please enter table to create/modify: ", 
		&mut table_name,
	);
	
	let _ =	create_table(db, &table_name)?; 

	inner_loop(db, &table_name)?;

	Ok(())
}

fn help_outer() {
	print!(
"Commands: 
    .tables - display all tables
    .select - select table to enter interactive mode
    .drop   - drop table(s)
    .help   - print help
    .quit   - quit mytodo
"
	)
}
pub fn run(db: Connection) -> Result<()> {
	'outer: loop {
		let mut command = String::new();

		command.clear();

		prompt_read_stdin(
			">>> ", 
			&mut command,
		);

		let command = command.trim(); 

		match command {
			"" => print!(""),
			"clear" =>  print!("\x1B[2J\x1B[1;1H"),
			".quit" => break 'outer,
			".tables" => display_tables(&db)?,  
			".select" => select_table(&db)?, 
			".help" => help_outer(), 
			_ => { 
				println!("{:?} is not a valid command", &command);
				help_outer();
			}
		}
	}

    Ok(())
}
