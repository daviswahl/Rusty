#![feature(plugin)]
#![feature(convert)]
//#![plugin(clippy)]

#![cfg(not(test))]
#[macro_use] extern crate rush;
extern crate copperline;
use rush::utils::*;
use rush::process::execute::interpret;
use rush::prompt::Prompt;
use rush::config::{check_alias,set_env_var};
use std::thread;
use std::fs::File;
use std::env::home_dir;
use std::io::{Read,Write};
use copperline::*;

fn main() {
    //Sets environment variables written in config file
    set_env_var();
    //Necessary to update as default prompt is not what we want
    //They were merely initialization values
    let prompt_spawn = thread::spawn(move || {
        let thread_prompt = Prompt::new();
        thread_prompt.print();
        thread_prompt
    });

    let input_spawn = thread::spawn(move || {
        let mut line_buffer = Copperline::new();
        //Pull history from file here

        let mut history = home_dir().expect("Home does not exist.");
        history.push(".rusty_history");

        //Creates file if it does not exist
        let _create = File::create(history.clone());

        //Opens file for reading and pushes them into history for the buffer
        let mut file = File::open(history).ok().expect("No History File");
        let mut buffer = String::new();
        let _result = file.read_to_string(&mut buffer);
        let lines = buffer.split("\n").collect::<Vec<&str>>();
        for i in lines {
            line_buffer.add_history(i.to_owned());
        }
        line_buffer
    });

    //Set up buffer to read inputs and History Buffer
    let mut input_buffer = input_spawn.join()
        .ok().expect("No InputBuffer made");
    let mut prompt = prompt_spawn.join()
        .ok().expect("No prompt made");
    //Loop to recieve and execute commands
    loop {

        let line = input_buffer.read_line(&prompt.get_user_p()).ok();
        if line.is_none(){
            continue;
        }
        let command = line.expect("Could not get line");
        input_buffer.add_history(command.clone());

        if command.starts_with("cd ") || command == "cd".to_owned() {
            cd::change_directory(command.trim_left_matches("cd").to_owned());
            prompt.update_cwd();
            prompt.update_prompt();

        } else if command.starts_with("clear") {
            let output = interpret(command);
            print!("{}", output);
            prompt.print();
            continue;

        } else if command.is_empty() {
            prompt.print();
            continue;

        } else if command.starts_with("exit") {
            break;

        } else {
            let alias = check_alias(command.clone());
            if alias.is_none() {
                let output = interpret(command);
                if !output.is_empty() {
                    println!("{}",output.trim());
                }

            } else {
                let mut vec = alias
                    .expect("Should have returned an unwrappable value")
                    .to_owned();

                //Removes alias and pushes the rest of the split onto
                //the string
                for (i,j) in command.split_whitespace()
                    .collect::<Vec<&str>>().iter().enumerate(){
                    if i != 0 {
                        vec.push_str(j);
                    }
                }

                let output =  interpret(vec);
                if !output.is_empty() {
                    println!("{}",output.trim());
                }

            }

        }
        //Updates the prompt for the next line
        prompt.print();
    }

    //Dump History to file here
    let mut history = home_dir().expect("Home does not exist.");
    history.push(".rusty_history");
    let mut file = File::create(history).ok().expect("No History File");
    for i in 0..input_buffer.get_current_history_length() {
        if i == 199 { //Write only the last 200 commands to history
            break;
        }
        let _unused_result = file.write(&input_buffer.remove_history_item(0)
                                        .expect("Out of bounds")
                                        .into_bytes().as_slice());
        let _other_result = file.write(&[10]); //add new line charachter
    }
}
