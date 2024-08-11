use rusqlite::{Connection, Result};
use std::io::{self, Write}; 
use crate::utilities;
use colored::Colorize;

#[derive(Debug)]
struct Todo {
	id: i32,
	user_order: f64, 
	numerator: i32, 
	denominator: i32, 
	todo: String, 
	status: i32, 
}

#[derive(Debug)] 
struct Fraction {
    numerator: i32, 
    denominator: i32, 
}

fn create_table(db: &Connection, table_name: &str) -> Result<usize>{
	let columns = vec![
		"user_order REAL",
		"numerator INTEGER",
		"denominator INTEGER",
		"todo TEXT NOT NULL",
		"status INTEGER",
	];

	db.execute(
        &*format!(
"CREATE TABLE IF NOT EXISTS {table_name} (
	id INTEGER PRIMARY KEY, 
	{}
)", 
			columns.join(",\n\t")
		),
        (), 
    )
}

fn get_latest_order(db: &Connection, table_name: &str) -> Option<i32>{
	let mut stmt = db.prepare(
		&*format!("SELECT user_order FROM {table_name} ORDER BY user_order DESC LIMIT 1")
	).expect("prep fail");
	
	let mut rows = stmt.query([]).expect("query fail");

	while let Some(row) = rows.next().expect("row fail") {
		let dec: f64 = row.get(0).expect("get fail");
		return Some((dec.floor() as i32) + 1);
	}

	None
}

fn insert(db: &Connection, table_name: &str) {
	let mut todo = String::new(); 

	utilities::prompt_read_stdin( 
		"-> Enter todo: ",
		&mut todo,
	);

	let todo = todo
		.trim()
		.to_string();

	let numerator = match get_latest_order(db, table_name) {
		Some(num) => num,
		None => 1, 
	}; 

	let entry = Todo {
		id: 0, 
		user_order: (numerator / 1) as f64,
		numerator, 
		denominator: 1,
		todo,
		status: 0,
	};
	
	if let Err(e) = db.execute(
		&*format!(
			"INSERT INTO {table_name} (
				user_order, 
				numerator, 
				denominator,
				todo, 
				status
			)
			Values (?1, ?2, ?3, ?4, ?5)"
		), 	
		(
			&entry.user_order,
			&entry.numerator,
			&entry.denominator,
			&entry.todo, 
			&entry.status
		),
	) { 
		println!("failed to add {}: {}", &entry.todo, e);
	} else {
		println!("added {}", &entry.todo); 
	}
}

fn get_total_count(db: &Connection, table_name: &str) -> i32 {
    let mut stmt = db.prepare(
        &*format!("SELECT COUNT(*) FROM {table_name}")
    ).expect("failed to get count"); 

    let mut rows = stmt.query([]).expect("query fail");

    while let Some(row) = rows.next().expect("row fail") {
        let dec: i32 = row.get(0).expect("get fail");
        return dec;
    }

    return 0
}

fn shift(db: &Connection, table_name: &str) { 
    let mut id = String::new(); 
    let mut new_order = String::new(); 
    let total = get_total_count(db, table_name); 
    
    utilities::prompt_read_stdin(
        "-> Enter id to move: ",
        &mut id, 
    );

    utilities::prompt_read_stdin(
        "-> Enter the new position:  ",
        &mut new_order, 
    );

    let new_order = match new_order.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            println!("{:?} is not a valid number", new_order);
            return; 
        }
    };
    
    if new_order < 1 || new_order > total {
        println!("index out of range");
        return;
    }

    let offset = new_order - 2; 

    let mut stmt = db.prepare(
        &*format!("SELECT numerator, denominator from {table_name} 
        ORDER BY user_order LIMIT 2 OFFSET {}", offset)
    ).expect("fail to get adjacent rows");  
   
    let frac_iter = stmt.query_map([], |row| {
        Ok(Fraction {
            numerator: row.get(0).expect("fail to get data"),
            denominator: row.get(1).expect("fail to get data"),
        })
    }).expect("fail to get fractions");  

    let mut buf: Vec<Fraction> = Vec::new(); 

    for frac in frac_iter {
        buf.push(frac.unwrap());
    } 
    
    let new_frac = match new_order {
        1  => { 
            let numerator = buf[0].numerator;
            let denominator = buf[0].denominator + 1;
            Fraction { numerator, denominator } 
        }
        num if num == total  => {
            let numerator = buf[1].numerator + 1;
            let denominator = buf[1].denominator;
            Fraction { numerator, denominator } 
        } 
        _ => {
            let numerator = buf[0].numerator + buf[1].numerator;
            let denominator = buf[0].denominator + buf[1].denominator; 
            Fraction { numerator, denominator } 
        }
    };

    db.execute(
        &*format!(
            "UPDATE {table_name} SET 
                numerator = {}, 
                denominator = {},
                user_order = {}
            WHERE id = {}",
            new_frac.numerator, 
            new_frac.denominator, 
            new_frac.numerator as f64 / new_frac.denominator as f64, 
            id,
        ),
        ()
    ).expect("moving failed"); 
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
	let mut stmt = db.prepare(
		&*format!(
			"SELECT
				id, 
				user_order, 
				numerator, 
				denominator,
				todo, 
				status
			FROM {table_name} ORDER BY user_order"
		)
	)?;
	let entry_iter = stmt.query_map([], |row| {
		Ok(Todo {
			id: row.get(0)?,
			user_order: row.get(1)?, 
			numerator: row.get(2)?, 
			denominator: row.get(3)?,
			todo: row.get(4)?,
			status: row.get(5)?,
		})
	})?;

    let mut order = 1; 
	for entry in entry_iter {
		let todo = entry.unwrap(); 
		println!("{}. (id: {}) {}", order, todo.id, todo.todo);
        order += 1;
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
	let _ =	create_table(
		db, 
		&table_name,
	)?; 

	let mut command = String::new();

	'inner: loop {
		command.clear();

		utilities::prompt_read_stdin(
			&*format!(
				"({}) >>> ", 
				&table_name
					.trim()
					.bold()
			), 
			&mut command,
		);
	
		let _ = io::stdout().flush(); 

		let command = command.trim();		

		match command {
			"" => print!(""),
			"clear" => print!("\x1B[2J\x1B[1;1H"),
			"quit" => break 'inner, 
			"add" => insert(db, table_name),
            "move" => shift(db, table_name),  
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
