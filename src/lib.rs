use rusqlite::{Connection, Result};

mod query; 
mod utilities;

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

	utilities::prompt_read_stdin(
		"-> Please enter table to create/modify: ", 
		&mut table_name,
	);
	
	let _ =	create_table(db, &table_name)?; 

	query::inner_loop(db, &table_name)?;

	Ok(())
}

fn drop_table(db: &Connection) -> Result<()> {
	let mut tables = String::new(); 

	utilities::prompt_read_stdin( 
		"-> Please enter table(s) to drop: ",
		&mut tables, 
	);


	for table in tables.split_whitespace() {
		db.execute(
			&*format!("DROP TABLE IF EXISTS {}", &*table),
			()
		)?;
		println!("dropped table: {}", table); 
	}

	Ok(())
}	

fn help_outer() {
	print!(
"Commands: 
    .tables - display all tables
    .select - select table to enter query mode
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

		utilities::prompt_read_stdin(
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
			".drop" => drop_table(&db)?,
			".help" => help_outer(), 
			_ => { 
				println!("{:?} is not a valid command", &command);
				help_outer();
			}
		}
	}

    Ok(())
}
