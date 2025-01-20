use crate::shared;
use franca_idl_parser::{variable_str, annotation_block, BasicContext, Rules};


#[test]
fn test_attribute_1() {
    let src = "attribute Playlist currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, 9));
}

#[test]
fn test_attribute_2() {
    let src = "currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_attribute_3() {
    let src = "_currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_attribute_4() {
    let src = "^currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_attribute_5() {
    let src = "^_currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, src.len() as u32));
}