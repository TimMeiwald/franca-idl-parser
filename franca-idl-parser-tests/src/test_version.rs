use crate::shared;
use franca_idl_parser::{version, BasicContext, Rules};

#[test]
fn test_version_1() {
    let src = r#"version {
		major 0
		minor 1		
	}"#;
    let result = shared(src, version::<BasicContext>, Rules::version); // Rule only matters for AST
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_version_2() {
    let src = r#"version {
		major     0
		minor         1		
	}"#;
    let result = shared(src, version::<BasicContext>, Rules::version); 
    assert_eq!(result, (true, src.len() as u32));
}

