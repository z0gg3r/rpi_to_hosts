// SPDX-FileCopyrightText: 2021 zocker <zockerfreunde03.info@gmx.de>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use getopt::Opt;

fn main() {
        let args: Vec<String> = std::env::args().collect();

        let mut opts = getopt::Parser::new(&args, "f:o:h");

        let mut file = String::new();
        let mut out = String::new();
        let mut help = false;

        loop {
                match opts
                        .next()
                        .transpose()
                        .expect("ERROR 3: Could not parse arguments!")
                {
                        None => break,
                        Some(opt) => match opt {
                                Opt('f', Some(arg)) => file = arg.clone(),
                                Opt('o', Some(arg)) => out = arg.clone(),
                                Opt('h', None) => help = true,
                                _ => unreachable!(),
                        },
                }
        }

        if help {
                print_usage(&args[0]);
        } else if file.is_empty() {
                let mut v: Vec<String> = vec![];

                loop {
                        let mut input = String::new();

                        let bytes = std::io::stdin()
                                .read_line(&mut input)
                                .expect("ERROR 4: Could not read from stdin!");
                        input = input.trim().to_string();

                        if bytes == 0 {
                                break;
                        }

                        v.push(input);
                }

                v = prefix(v);

                if out.is_empty() {
                        for line in v {
                                println!("{}", line);
                        }
                } else {
                        let x = write_file(out.clone(), combine(v));
                        let x = if x.is_ok() {
                                String::new()
                        } else {
                                x.unwrap_err().to_string()
                        };

                        if !x.is_empty() {
                                eprintln!("{}", x);
                        }
                }
        } else {
                let input = read_file(file.clone());
                let input = if input.is_ok() {
                        input.unwrap()
                } else {
                        String::new()
                };
                if !input.is_empty() {
                        let v = split(input);

                        let v = prefix(v);

                        if out.is_empty() {
                                for line in v {
                                        println!("{}", line);
                                }
                        } else {
                                let x = write_file(out.clone(), combine(v));
                                let x = if x.is_ok() {
                                        String::new()
                                } else {
                                        x.unwrap_err().to_string()
                                };

                                if !x.is_empty() {
                                        eprintln!("{}", x);
                                }
                        }
                }
        }
}

fn print_usage(s: &str) {
        println!("{} [-f INPUT] [-o OUTPUT]", s);
}

fn split(s: String) -> Vec<String> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut splits: Vec<String> = Vec::with_capacity(lines.len());

        for line in lines {
                splits.push(String::from(line));
        }

        splits
}

fn combine(v: Vec<String>) -> String {
        let mut s = String::new();

        for x in v {
                s += &x;
                s += "\n";
        }

        s
}

fn prefix(v: Vec<String>) -> Vec<String> {
        let mut p: Vec<String> = vec![];

        let host = String::from("0.0.0.0 ");

        for x in v {
                dbg!(&x);
                if x.starts_with('#') {
                        p.push(x);
                } else if !x.is_empty() {
                        p.push(host.clone() + &x);
                } 
        }

        p
}

fn read_file(
        _path: String,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
        let path = std::path::Path::new(&_path);

        if !path.exists() {
                eprintln!("ERROR 1: {} is not a valid path!", _path);
        }

        if !path.is_file() {
                eprintln!("ERROR 2: {} is valid, but not a file!", _path);
        }

        let file = std::fs::read_to_string(path)?;

        Ok(file)
}

fn write_file(
        _path: String,
        contents: String,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
        let path = std::path::Path::new(&_path);

        std::fs::write(path, contents)?;

        Ok(String::new())
}
