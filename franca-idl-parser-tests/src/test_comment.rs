use crate::shared;
use franca_idl_parser::{comment, annotation_block, BasicContext, Rules};


#[test]
fn test_comment_1() {
    let src = "// ***\n";
    let result = shared(src, comment::<BasicContext>, Rules::comment); 
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_comment_2() {
    let src = "// ***\r\n";
    let result = shared(src, comment::<BasicContext>, Rules::comment); 
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_comment_3() {
    let src = "// ***";
    let result = shared(src, comment::<BasicContext>, Rules::comment); 
    assert_eq!(result, (false, 0));
}
#[test]
fn test_comment_4() {
    let src = "// ***   \t   \r\n";
    let result = shared(src, comment::<BasicContext>, Rules::comment); 
    assert_eq!(result, (true, src.len() as u32));
}