use crate::shared;
use franca_idl_parser::{variable_str, annotation_block, BasicContext, Rules};


#[test]
fn test_attribute_1() {
    let src = "attribute Playlist currentPlaylist";
    let result = shared(src, variable_str::<BasicContext>, Rules::Grammar); // Rule only matters for AST
    assert_eq!(result, (true, 9));
}