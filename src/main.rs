use std::io::{self, BufRead, Stdin, Write};

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

#[derive(Default)]
struct Statement {
    kind: Option<StatementType>,
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
        (*statement).kind = Some(StatementType::StatementInsert);

        // TODO this enum is useless lol just use Result or Option here!
        return PrepareResult::PrepareSuccess;
    }

    if buffer.starts_with("select") {
        (*statement).kind = Some(StatementType::StatementSelect);

        // TODO this enum is useless lol just use Result or Option here!
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

        let mut statement: Statement = Statement { kind: None };

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::PrepareSuccess => (),
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input_buffer);
                continue;
            }
        }

        execute_statement(&statement);
        println!("Executed.");
    }
}
