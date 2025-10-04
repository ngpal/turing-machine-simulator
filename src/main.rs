use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write, stdin, stdout};
use std::path::Path;

fn parse_transition(line: &str) -> Option<(String, char, String, char, char)> {
    // trim whitespace
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // expected format: (q0, a) -> (q1, b, <)
    // we'll parse this manually instead of regex
    let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    if parts.len() != 2 {
        return None;
    }

    // left side: (q0, a)
    let left = parts[0].trim_matches(|c| c == '(' || c == ')');
    let left_parts: Vec<&str> = left.split(',').map(|s| s.trim()).collect();
    if left_parts.len() != 2 {
        return None;
    }

    let from_state = left_parts[0].to_string();
    let read_symbol = left_parts[1].chars().next()?;

    // right side: (q1, b, >)
    let right = parts[1].trim_matches(|c| c == '(' || c == ')');
    let right_parts: Vec<&str> = right.split(',').map(|s| s.trim()).collect();
    if right_parts.len() != 3 {
        return None;
    }

    let to_state = right_parts[0].to_string();
    let write_symbol = right_parts[1].chars().next()?;
    let direction = right_parts[2].chars().next()?;

    Some((from_state, read_symbol, to_state, write_symbol, direction))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut transition_table = HashMap::new();
    let mut finals = Vec::new();
    if args.len() < 2 {
        eprintln!("usage: {} <inputfile>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();
    // first line: final states (we’ll just read and print them)
    if let Some(Ok(final_states)) = lines.next() {
        println!("final states: {}", final_states.trim());
        finals.extend(final_states.split(",").map(|s| s.trim().to_string()));
    }

    for line in lines {
        let line = line?;
        if let Some((from, read, to, write, dir)) = parse_transition(&line) {
            if let Some((prev_to, prev_write, prev_dir)) =
                transition_table.insert((from.clone(), read), (to.clone(), write, dir))
            {
                eprintln!(
                    "warning: transition ({from}, {read}) -> ({prev_to}, {prev_write}, {prev_dir}) remapped to ({to}, {write}, {dir})"
                )
            }
        } else if !line.trim().is_empty() {
            eprintln!("warning: could not parse line: {}", line);
        }
    }

    println!("\nparsed {} transitions:", transition_table.len());
    for ((from, read), (to, write, dir)) in transition_table.iter() {
        println!("({from} , {read}) -> ({to} , {write} , {dir})",);
    }

    // execute machine
    println!("\nstarting from q0");
    print!("Enter input string: ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let mut tape = ['_'; 256];
    let mut head: u8 = 0;

    // copy input into tape
    for (i, ch) in input.chars().enumerate() {
        tape[i] = ch;
    }

    let mut cur_state = String::from("q0");

    loop {
        let cur_char = tape[head as usize];
        if let Some((to, write, dir)) = transition_table.get(&(cur_state.clone(), cur_char)) {
            cur_state = to.to_string();
            tape[head as usize] = *write;

            head = match dir {
                '>' => head.wrapping_add(1),
                '<' => head.wrapping_sub(1),
                _ => unreachable!(),
            };
        } else {
            if finals.contains(&cur_state) {
                println!("HALTED: ACCEPTED")
            } else {
                println!("HALTED: ERROR");
            }

            break;
        }
    }

    Ok(())
}
