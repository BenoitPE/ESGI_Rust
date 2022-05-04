use crate::Instruction::*;
use std::io::{self, Read};
use substring::Substring;

#[derive(Debug)]
enum Instruction {
    Plus,
    Moins,
    Droite,
    Gauche,
    Affiche,
    Lis,
    Boucle(Vec<Instruction>),
}
fn interpreteur(mut memoire: &mut Vec<i32>, instructions: &Vec<Instruction>, current: &mut usize) {
    if 0 == memoire.len() {
        memoire.push(0)
    }
    for i in instructions {
        match i {
            Plus => memoire[*current] += 1,
            Moins => memoire[*current] -= 1,
            Droite => {
                if (*current + 1) < memoire.len() {
                    *current += 1
                } else {
                    memoire.push(0);
                    *current += 1
                }
            }
            Gauche => {
                if (*current - 1) == 0 {
                    *current = 0
                } else {
                    *current -= 1
                }
            }
            Affiche => print!(
                "{}",
                std::char::from_u32(memoire[*current] as u32).unwrap_or('?')
            ),
            Lis => {
                let mut character = [0];
                match io::stdin().read(&mut character) {
                    Ok(_) => memoire[*current] = character[0] as i32,
                    Err(_) => println!("Erreur de lecture"),
                }
            }
            Boucle(x) => {
                while memoire[*current] != 0 {
                    interpreteur(&mut memoire, &x, current)
                }
            }
        }
    }
}
fn parse(source: String) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;
    let mut i = 0;
    for symbol in source.chars() {
        if loop_stack == 0 {
            match symbol {
                '>' => program.push(Droite),
                '<' => program.push(Gauche),
                '+' => program.push(Plus),
                '-' => program.push(Moins),
                '.' => program.push(Affiche),
                ',' => program.push(Lis),
                '[' => {
                    loop_start = i;
                    loop_stack += 1;
                }
                _ => (),
            };
        } else {
            match symbol {
                '[' => {
                    loop_stack += 1;
                }
                ']' => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Boucle(parse(
                            source.substring(loop_start + 1, i).to_string(),
                        )));
                    }
                }
                _ => (),
            }
        }
        i += 1
    }
    program
}

fn main() {
    let mut mem3 = vec![];
    let mut opcodes = "".to_string();

    match std::fs::read_to_string("src/programs/hanoi.bf".to_string()) {
        Ok(x) => opcodes = x,
        Err(_) => (),
    }
    let program = parse(opcodes);
    interpreteur(&mut mem3, &program, &mut 0);
}
