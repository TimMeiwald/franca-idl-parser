use test_grammar_proc_macro::test_grammar_files_in_dir;
use std::{
    fs::{self, File},
    io::Read,
};

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




test_grammar_files_in_dir!("grammar_test_files");