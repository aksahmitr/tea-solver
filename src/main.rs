use clipboard_win::{formats, set_clipboard};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

enum State {
    Red,
    Yellow,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let game_state = match args.get(1).cloned().unwrap_or_default().as_str() {
        "yellow" => State::Yellow,
        _ => State::Red,
    };

    let file = File::open("words.txt")?;
    let reader = BufReader::new(file);

    let mut data: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for word in reader.lines().flat_map(|e| e) {
        if word.len() < 3 {
            continue;
        }
        for i in 0..word.len() - 2 {
            let key = word.get(i..i + 3).unwrap().to_string();
            let outer = data.entry(key).or_insert(Vec::new());
            outer.resize(std::cmp::max(outer.len(), word.len() + 1), Vec::new());
            outer.get_mut(word.len()).unwrap().push(word.clone());
        }
    }

    loop {
        println!("Key: ");
        let mut key = String::new();
        io::stdin()
            .read_line(&mut key)
            .expect("failed to read line");

        key = key.trim().to_string();

        if key == "exit" {
            break;
        }

        println!();

        match data.get(&key) {
            Some(val) => match game_state {
                State::Red => {
                    let pad = 1 + val.len() / 10;
                    let len = val.len();
                    for j in (1..len).rev() {
                        let words = val.get(j).unwrap();
                        if words.len() > 0 {
                            print!("{:>pad$}   ", j);
                            for i in 0..std::cmp::min(5, words.len()) {
                                print!("{:<len$} ", words.get(i).unwrap());
                            }
                            println!()
                        }
                    }
                }
                State::Yellow => {
                    let limit = 2000;
                    let mut count = 0;
                    let mut result: String = String::default();
                    'out: for j in 1..val.len() {
                        let words = val.get(j).unwrap();
                        for i in 0..words.len() {
                            if count + j > limit {
                                break 'out;
                            }
                            result.push_str(words.get(i).unwrap());
                            result.push(' ');
                            count += j + 1;
                        }
                    }
                    result.pop();
                    println!("{result}");

                    set_clipboard(formats::Unicode, result).expect("failed to set clipboard");
                }
            },
            None => println!("---"),
        }
        println!();
    }
    Ok(())
}
