#![allow(unused_must_use)] //supresses warning from use of env::var("Home")
use std::path::{Path,PathBuf};
use std::env;
use std::fs::PathExt; //Use of exists() is considered unstable. Might break in the future

pub fn change_directory(input: Vec<&str>){
    if input.is_empty(){
        env::set_current_dir(Path::new(env::var("HOME").unwrap().as_str()));
    } else{
        let mut buffer = PathBuf::new();
        for i in input {
            if i.contains("~") {
                let i_split = i.split("~").next();
                buffer.push(Path::new(env::var("HOME").unwrap().as_str()));
                if i_split.is_some(){
                    buffer.push(Path::new(i_split.unwrap()));
                }
            } else {
                buffer.push(Path::new(i));
            }
        }
        let dir = buffer.as_path();
        if dir.is_relative(){
            let mut temp = PathBuf::new();
            temp.push(dir.parent().unwrap());
            temp.push(dir);
            let path = temp.as_path();
            if path.exists(){
                env::set_current_dir(temp.as_path()).unwrap();
            } else {
                println!("Invalid path or input");
            }
        } else {
            if dir.exists(){
                env::set_current_dir(dir).unwrap();
            } else {
                println!("Invalid path or input");
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use super::*;

    #[test]
    #[should_panic]
    fn test_change_directory_ok(){
        let vec = vec!["/"];
        let dir = Path::new("/tmp").to_str();
        change_directory(vec);
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

    #[test]
    fn test_change_directory_fail(){
        let vec = vec!["/"];
        let dir = Path::new("/").to_str();
        change_directory(vec);
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

}

