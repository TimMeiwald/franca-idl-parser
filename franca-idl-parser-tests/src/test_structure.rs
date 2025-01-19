use crate::shared;
use franca_idl_parser::{structure, annotation_block, BasicContext, Rules};


#[test]
fn test_structure_1() {
    let src = r#"<** @description: Duration in hours, minutes and seconds. **>
	struct Duration {
		UInt8 hours
		UInt8 minutes
		UInt8 seconds
	}"#;
    let result = shared(src, structure::<BasicContext>, Rules::structure); 
    assert_eq!(result, (true, src.len() as u32));
}


#[test]
fn test_structure_2() {
    let src = r#"struct Duration {
		UInt8 hours
		UInt8 minutes
		UInt8 seconds
	}
    "#;
    let result = shared(src, structure::<BasicContext>, Rules::structure); 
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_structure_3() {
    let src = r#"struct Duration {
		UInt8 hours
		UInt8 minutes
		UInt8[] seconds
	}
    "#;
    let result = shared(src, structure::<BasicContext>, Rules::structure); 
    assert_eq!(result, (true, src.len() as u32));
}