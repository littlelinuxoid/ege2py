#![allow(dead_code)]

use std::{env::args, process::exit};

#[derive(Debug)]
enum Token {
    Rep,
    Tailup,
    Back,
    Closebr,
    Forw,
    Left,
    Num(u16),
    Openbr,
    Taildown,
    Right,
    Mult,
}
fn produce_py(a: Vec<Token>) -> String {
    let mut answer =
        String::from("from turtle import *\ntracer(0)\nm = 1\nscreensize(2000,2000)\n");
    let mut indent = 0;
    for (i, t) in a.iter().enumerate() {
        match t {
            Token::Mult if i == 0 => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(format!("m = {num}\n").as_str());
                } else {
                    eprintln!("Масштаб Задан некорректно");
                    exit(1);
                }
            }
            Token::Mult => {
                eprintln!("Масштаб должен быть задан в начале программы первой строкой");
                exit(1)
            }
            Token::Rep => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(
                        format!("{}for i in range({}):\n", "\t".repeat(indent), num).as_str(),
                    );
                } else {
                    eprintln!("Команда 'Повтори' не получила количества итераций");
                    exit(1);
                }
                indent += 1;
            }
            Token::Left => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(format!("{}lt(m * {})\n", "\t".repeat(indent), num).as_str());
                } else {
                    eprintln!("Команда 'Налево' не получила аргумента");
                    exit(1);
                }
            }
            Token::Closebr => indent -= 1,
            Token::Right => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(format!("{}rt(m * {})\n", "\t".repeat(indent), num).as_str());
                } else {
                    eprintln!("Команда 'Направо' не получила аргумента");
                    exit(1);
                }
            }
            Token::Forw => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(format!("{}fd(m * {})\n", "\t".repeat(indent), num).as_str());
                } else {
                    eprintln!("Команда 'Вперед' не получила аргумента");
                    exit(1);
                }
            }
            Token::Openbr => {}
            Token::Back => {
                if let Token::Num(num) = a[i + 1] {
                    answer.push_str(format!("{}bk(m * {})\n", "\t".repeat(indent), num).as_str());
                } else {
                    eprintln!("Команда 'Назад' не получила аргумента");
                    exit(1);
                }
            }
            Token::Tailup => {
                answer.push_str(format!("{}up()\n", "\t".repeat(indent)).as_str());
            }
            Token::Taildown => {
                answer.push_str(format!("{}down()\n", "\t".repeat(indent)).as_str());
            }
            Token::Num(_) => {}
        }
    }
    answer.push_str("up()\n");
    answer.push_str(
        "\nfor x in range(-50,50):\n\tfor y in range(-50,50):\n\t\tgoto(x *m , y * m)\n\t\tdot(2, 'blue')\n",
    );
    answer.push_str("done()");
    answer
}
fn tokenize(target: String) -> Vec<Token> {
    let mut answer: Vec<Token> = vec![];
    for mut i in target.split_whitespace() {
        let mut bracket = false;
        if i.starts_with("[") {
            answer.push(Token::Openbr);
            i = unsafe { i.strip_prefix("[").unwrap_unchecked() };
        } else if i.ends_with("]") {
            bracket = true;
            i = unsafe { i.strip_suffix("]").unwrap_unchecked() };
        }

        if let Ok(num) = i.parse::<u16>() {
            answer.push(Token::Num(num));
        }
        match i {
            "Масштаб" => answer.push(Token::Mult),
            "Повтори" => answer.push(Token::Rep),
            "Поднять" => answer.push(Token::Tailup),
            "Опустить" => answer.push(Token::Taildown),
            "Налево" => answer.push(Token::Left),
            "Направо" => answer.push(Token::Right),
            "Вперёд" => answer.push(Token::Forw),
            "Назад" => answer.push(Token::Back),
            "хвост" => {}
            &_ => {}
        }
        if bracket {
            answer.push(Token::Closebr);
        }
    }
    answer
}
fn main() {
    let mut a = args();
    let p = a.nth(1);
    if let Some(path) = p {
        let target = std::fs::read_to_string(path).unwrap();
        let tokens = tokenize(target);
        println!("{}", produce_py(tokens))
    } else {
        eprintln!("Укажите имя файла!")
    }
}
