use crate::shared;
use franca_idl_parser::{map, annotation_block, BasicContext, Rules};

#[test]
fn test_map_1() {
    let src = r#"<** @description: Set current repeat mode. **>
	map aMapKey08 { UInt64 to Int16 }"#;
    let result = shared(src, map::<BasicContext>, Rules::map); 
    assert_eq!(result, (true, src.len() as u32));
}