use rusqlite::{Connection, Result};
use std::io::{self, Write}; 
use crate::utilities;


#[derive(Debug)]
struct Todo {
	id: i32, 
	todo: String, 
	priority: i32, 
	status: i32, 
}

fn insert(db: &Connection, table_name: &str) {
	let mut todo = String::new(); 

	utilities::prompt_read_stdin( 
		"-> Enter todo: ",
		&mut todo,
	);

	let todo = todo.trim().to_string();

	let mut priority = String::new(); 

	let priority = loop {
		priority.clear(); 
		utilities::prompt_read_stdin( 
			"-> Enter priority value (0 - 2, 2 being most urgent): ",
			&mut priority,
		);

		match (*priority).trim().parse::<i32>() {
			Ok(num) => {
				break num;
			}
			Err(e) => println!("cannot parse a non integer: {e} "), 
		};
	}; 

	let entry = Todo {
		id: 0, 
		todo,
		priority,	
		status: 0,
	};
	
	if let Err(e) = db.execute(
		&*format!("INSERT INTO {table_name} (todo, priority, status) Values (?1, ?2, ?3)"), 	
		(&entry.todo, &entry.priority, &entry.status),
	) { 
		println!("failed to add {}: {}", &entry.todo, e);
	} else {
		println!("added {}", &entry.todo); 
	}
}

fn delete(db: &Connection, table_name: &str) {
	let mut ids = String::new(); 

	utilities::prompt_read_stdin( 
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
	let mut stmt = db.prepare(&*format!("SELECT id, todo, priority, status  FROM {table_name}"))?;
	let entry_iter = stmt.query_map([], |row| {
		Ok(Todo {
			id: row.get(0)?,
			todo: row.get(1)?,
			priority: row.get(2)?,
			status: row.get(3)?,
		})
	})?;

	for entry in entry_iter {
		println!("Found entry {:?}", entry.unwrap());
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
    quit - quit query mode
"
	)
}

pub fn inner_loop(db: &Connection, table_name: &str) -> Result<()> {
	'inner: loop {
		let mut command = String::new();

		command.clear();

		utilities::prompt_read_stdin(
			&*format!("({}) >>> ", &table_name.trim()), 
			&mut command,
		);
	
		let _ = io::stdout().flush(); 

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

