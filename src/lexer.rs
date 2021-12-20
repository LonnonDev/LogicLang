use std::fmt::{self, write};

use regex::Regex;
use lazy_static::lazy_static;

enum Tokens {
    Gate(String, u8, u8),
    Name(String),
    Input,
    Output,
    DeclareVar,
    ReferenceVar(String),
    Code,
    StartScope,
    EndScope,
    Assignment,
    SemiColon,
    Function(String, u8, u8),
    LPeren,
    RPeren,
}

impl fmt::Debug for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tokens::Gate(name, inputs, outputs) => write!(f, "Gate({:?}, {:?}, {:?})", name, inputs, outputs),
            Tokens::Name(name) => write!(f, "Name({:?})", name),
            Tokens::Input => write!(f, "Input"),
            Tokens::Output => write!(f, "Output"),
            Tokens::DeclareVar => write!(f, "DeclareVar"),
            Tokens::ReferenceVar(var) => write!(f, "ReferenceVar({:?})", var),
            Tokens::Code => write!(f, "Code"),
            Tokens::StartScope => write!(f, "StartScope"),
            Tokens::EndScope => write!(f, "EndScope"),
            Tokens::Assignment => write!(f, "Assignment"),
            Tokens::SemiColon => write!(f, "SemiColon"),
            Tokens::Function(name, inputs, outputs) => write!(f, "Function({:?}, {:?}, {:?})", name, inputs, outputs),
            Tokens::LPeren => write!(f, "LPeren"),
            Tokens::RPeren => write!(f, "RPeren")
        }
    }
}

impl PartialEq for Tokens {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Gate(l0, l1, l2), Self::Gate(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            (Self::Name(l0), Self::Name(r0)) => l0 == r0,
            (Self::ReferenceVar(l0), Self::ReferenceVar(r0)) => l0 == r0,
            (Self::Function(l0, l1, l2), Self::Function(r0, r1, r2)) => l0 == r0 && l1 == r1 && l2 == r2,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub fn tokenize(content: String) {
    let mut tokenized_content: Vec<Tokens> = vec![];

    let mut currentcontent = String::new();
    let mut x = 0;

    // Compile the regex's only once
    lazy_static! {
        // Gates
        static ref GATE: Regex = Regex::new(r"Gate [a-z, A-Z]*\(\d+, \d+\)").unwrap();
        static ref GATE_NAME: Regex = Regex::new(r"[a-z, A-Z]*").unwrap();
        static ref GATE_NUMBERS: Regex = Regex::new(r"\d+").unwrap();

        // Name
        static ref NAME: Regex = Regex::new(r" [a-z, A-Z]+;| [a-z, A-Z]+ ").unwrap();

        // Scopes
        static ref SCOPESTART: Regex = Regex::new(r"\{").unwrap();
        static ref SCOPEEND: Regex = Regex::new(r"\}").unwrap();

        // Inputs
        static ref INPUT: Regex = Regex::new(r"in").unwrap();

        // Outputs
        static ref OUTPUT: Regex = Regex::new(r"out").unwrap();

        // Variable Declaration
        static ref DECLAREVAR: Regex = Regex::new(r"let").unwrap();

        // Assignment 
        static ref ASSIGNMENT: Regex = Regex::new(r"=").unwrap();

        // SemiColon
        static ref SEMICOLON: Regex = Regex::new(";").unwrap();
    }

    while x != content.as_bytes().len() {
        currentcontent.push(content.as_bytes()[x] as char);
        
        // check if it's gate syntax
        if GATE.is_match(&currentcontent) {
            let mut numbers = vec![];
            let mut name = vec![];
            // get the input and output numbers
            for cap in GATE_NUMBERS.captures_iter(&currentcontent) {
                numbers.push(cap[0].parse::<u8>().unwrap())
            }
            // get the name of the game
            for cap in GATE_NAME.captures_iter(&currentcontent) {
                let mut cap_bytes = cap[0].to_string().as_bytes().to_vec();
                // removes the "Gate " part
                for x in 0..=4 {
                    cap_bytes.remove(0);
                }
                name.push(String::from_utf8(cap_bytes).unwrap());
                break;
            }

            tokenized_content.push(Tokens::Gate(name[0].to_string(), numbers[0], numbers[1]));
            currentcontent = String::new()
        }
        if tokenized_content.len() != 0 {
            // name
            if 
                NAME.is_match(&currentcontent) 
                && tokenized_content[tokenized_content.len() - 1] == Tokens::DeclareVar
                || tokenized_content[tokenized_content.len() - 1] == Tokens::Input
                || tokenized_content[tokenized_content.len() - 1] == Tokens::Output 
            {
                for cap in NAME.captures_iter(&currentcontent) {
                    let mut cap_bytes = cap[0].to_string();
                    cap_bytes.remove(0);
                    cap_bytes.remove(cap_bytes.len() - 1);
                    tokenized_content.push(Tokens::Name(cap_bytes.clone()));
                    currentcontent = cap[cap.len() - 1].to_string();
                    break;
                }
            }
        }
        
        // check for scopes
        if SCOPESTART.is_match(&currentcontent) {
            tokenized_content.push(Tokens::StartScope);
            currentcontent = String::new();
        }
        if SCOPEEND.is_match(&currentcontent) {
            tokenized_content.push(Tokens::EndScope);
            currentcontent = String::new();
        }

        // inputs
        if INPUT.is_match(&currentcontent) {
            tokenized_content.push(Tokens::Input);
            currentcontent = String::new();
        }

        // outputs
        if OUTPUT.is_match(&currentcontent) {
            tokenized_content.push(Tokens::Output);
            currentcontent = String::new();
        }

        // variables
        if DECLAREVAR.is_match(&currentcontent) {
            tokenized_content.push(Tokens::DeclareVar);
            currentcontent = String::new();
        }

        // assignment
        if ASSIGNMENT.is_match(&currentcontent) {
            tokenized_content.push(Tokens::Assignment);
            currentcontent = String::new();
        }

        // semicolon
        if SEMICOLON.is_match(&currentcontent) {
            tokenized_content.push(Tokens::SemiColon);
            currentcontent = String::new();
        }
        
        x += 1;
    }
    println!("{:?}", tokenized_content);
}