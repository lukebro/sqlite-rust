use std::io::{self, BufRead, Stdin, Write};

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareSyntaxError,
    PrepareUnrecognizedStatement,
}

#[derive(Debug)]
enum StatementType {
    StatementInsert,
    StatementSelect,
}

#[derive(Default, Debug)]
struct Row {
    id: u32,
    username: String,
    email: String,
}

#[derive(Default, Debug)]
struct Statement {
    kind: Option<StatementType>,
    row_to_insert: Option<Row>,
}

fn execute_statement(statement: &Statement) -> () {
    match statement.kind.as_ref().unwrap() {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.");
        }

        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
    }
}

fn do_meta_command(buffer: &String) -> MetaCommandResult {
    if buffer == ".exit" {
        std::process::exit(0);
    } else {
        MetaCommandResult::MetaCommandUnrecognizedCommand
    }
}

fn prepare_statement(buffer: &String, statement: &mut Statement) -> PrepareResult {
    if buffer.starts_with("insert") {
        statement.kind = Some(StatementType::StatementInsert);

        let args = buffer.split_whitespace().skip(1).collect::<Vec<&str>>();

        match args.as_slice() {
            [id, username, email] => {
                // TODO: this can be much better in rust version
                let id = match id.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {
                        return PrepareResult::PrepareSyntaxError;
                    }
                };

                statement.row_to_insert = Some(Row {
                    id: id,
                    username: username.to_string(),
                    email: email.to_string(),
                });

                return PrepareResult::PrepareSuccess;
            }
            _ => {
                return PrepareResult::PrepareSyntaxError;
            }
        }
    }

    if buffer.starts_with("select") {
        (*statement).kind = Some(StatementType::StatementSelect);

        // TODO: this enum is useless lol just use Result or Option here!
        return PrepareResult::PrepareSuccess;
    }

    return PrepareResult::PrepareUnrecognizedStatement;
}

fn print_prompt() {
    print!("db > ");
    io::stdout().lock().flush().unwrap();
}

fn read_input(stdin: &Stdin, buffer: &mut String) {
    let mut handle = stdin.lock();
    handle.read_line(buffer).unwrap();

    // remove return character
    buffer.truncate(buffer.len() - 1);
}

fn main() {
    let mut input_buffer = String::new();
    let stdin = io::stdin();

    loop {
        input_buffer.clear();
        print_prompt();
        read_input(&stdin, &mut input_buffer);

        if let Some('.') = input_buffer.chars().nth(0) {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input_buffer);
                    continue;
                }
            }
        }

        let mut statement: Statement = Statement::default();

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::PrepareSuccess => {

            }
            PrepareResult::PrepareSyntaxError => {
                println!("Unknown syntax error of '{}'.", input_buffer);
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input_buffer);
                continue;
            }
        }

        execute_statement(&statement);
        println!("Executed.");
    }
}
