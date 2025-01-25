use test_grammar_proc_macro::test_grammar_files_in_dir;
use std::{
    fs::{self, File},
    io::Read,
};

macro_rules! test_grammar_file {
    ($path:expr) => {{
        let path = $path;
        let file_name = std::path::Path::new(&path).file_name().expect("Should exist");
        
        println!("{:?}", file_name);
        let file_path = $path;
        let src = fs::read_to_string(file_path);
        let src = match src {
            Err(err) => {
                panic!("Error: {:?}", err)
            }
            Ok(src) => src,
        };
        shared(&src, grammar::<BasicContext>, Rules::Grammar)
    }
    }
}


use crate::shared;
use franca_idl_parser::{grammar, BasicContext, Rules};
#[test]
fn test_grammar_1() {
    let src = "package org.javaohjavawhyareyouso
	interface endOfPlaylist { }	";
    let result = shared(src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_grammar_2() {
    let file_path = "../grammar_test_files/BluetoothManager.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_grammar_3() {
    let file_path = "../grammar_test_files/CommonTypes.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_grammar_4() {
    let file_path = "../grammar_test_files/MediaPlayer.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_grammar_5() {
    let file_path = "../grammar_test_files/PowerManagement.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_grammar_6() {
    let file_path = "../grammar_test_files/speller.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_grammar_7() {
    let file_path = "../grammar_test_files/SystemdManager.fidl";
    let src = fs::read_to_string(file_path);
    let src = match src {
        Err(err) => {
            panic!("Error: {:?}", err)
        }
        Ok(src) => src,
    };
    let result = shared(&src, grammar::<BasicContext>, Rules::Grammar);
    assert_eq!(result, (true, src.len() as u32));
}


#[test]
fn test_grammar_all_test() {
    let file_path = "../grammar_test_files/";
    let files = std::fs::read_dir(file_path).expect("Expected grammar_test_files dir exists");

    for path in files{
        // Actually read and test each file
        
        let result = test_grammar_file!(path.as_ref().expect("File should exist").path());
        println!("{:?}", result)
        
    }

   
}

test_grammar_files_in_dir!("../grammar_test_files/");