use crate::Instruction::*;
use std::{
    fmt::Error,
    io::{self, BufRead, Read},
};

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

fn interpreteur(memoire: &mut Vec<i32>, instructions: &Vec<Instruction>, pos: usize) {
    let mut current = pos;

    for i in instructions {
        match i {
            Plus => memoire[current] += 1,
            Moins => memoire[current] -= 1,
            Droite => {
                if current + 1 < memoire.len() {
                    current += 1;
                } else {
                    memoire.push(0);
                    current += 1;
                }
            }
            Gauche => {
                if current - 1 != 0 {
                    current -= 1;
                } else {
                    current = 0;
                }
            }
            Affiche => print!(
                "{}",
                std::char::from_u32(memoire[current] as u32).unwrap_or('?')
            ),
            Lis => {
                let mut buffer = [0];
                match io::stdin().read(&mut buffer) {
                    Ok(x) => memoire[current] = buffer[0] as i32,
                    Err(x) => println!("Erreur de lecture"),
                }
            }
            Boucle(x) => {
                while memoire[current] != 0 {
                    interpreteur(memoire, x, current);
                }
            }
        }
    }
}

fn parse(s: &String) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut isOk = true;
    for c in s.chars() {
        match c {
            '+' => instructions.push(Plus),
            '-' => instructions.push(Moins),
            '<' => instructions.push(Gauche),
            '>' => instructions.push(Droite),
            '.' => instructions.push(Affiche),
            ',' => instructions.push(Lis),
            '[' => instructions.push(Affiche),
            ']' => instructions.push(Affiche),
            _ => isOk = false,
        }
    }

    if !isOk {
        Err("Erreur: Conversion impossible".to_string())
    } else {
        Ok(instructions)
    }
}

fn main() {
    /*let mut mem = vec![72, 101, 108, 108, 111];
    let instructions = vec![
        Lis, Plus, Affiche, Lis, Plus, Affiche, Lis, Plus, Lis, Plus, Affiche,
    ];

    interpreteur(&mut mem, &instructions, 0);*/

    /*
    let mut mem = vec![10, 15];
    let instructions = vec![Boucle(vec![Moins, Droite, Plus, Gauche])];

    interpreteur(&mut mem, &instructions, 0);

    println!("{:?}", mem);
     */

    let instructions = std::fs::read_to_string("programs/hello.bf".to_string());

    //let instructions = parse(&"+-<>.,".to_string());
    println!("{:?}", instructions);
}
