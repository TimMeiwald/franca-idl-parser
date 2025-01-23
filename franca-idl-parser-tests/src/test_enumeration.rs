use crate::shared;
use franca_idl_parser::{enumeration, annotation_block, BasicContext, Rules};


#[test]
fn test_enumeration_1() {
    let src = r#"<** @description : Repeat modes for playback. **>
	enumeration RepeatMode {
		MODE_REPEAT_NONE   = 0
		MODE_REPEAT_SINGLE = 1
		MODE_REPEAT_ALL    = 2
	}"#;
    let result = shared(src, enumeration::<BasicContext>, Rules::enumeration); 
    assert_eq!(result, (true, src.len() as u32));
}


#[test]
fn test_enumeration_2() {
    let src = r#"<** @description : Repeat modes for playback. **>
	enumeration RepeatMode {
		MODE_REPEAT_NONE
		MODE_REPEAT_SINGLE
		MODE_REPEAT_ALL
	}"#;
    let result = shared(src, enumeration::<BasicContext>, Rules::enumeration); 
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_enumeration_3() {
    let src = r#"enumeration RepeatMode {
		MODE_REPEAT_NONE
		MODE_REPEAT_SINGLE
		MODE_REPEAT_ALL
	}"#;
    let result = shared(src, enumeration::<BasicContext>, Rules::enumeration); 
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_enumeration_4() {
    let src = r#"<** @description : Repeat modes for playback. **>
	enumeration RepeatMode {
		MODE_REPEAT_NONE
		MODE_REPEAT_SINGLE,
		MODE_REPEAT_ALL
	}"#;
    let result = shared(src, enumeration::<BasicContext>, Rules::enumeration); 
    assert_eq!(result, (true, src.len() as u32));
}


#[test]
fn test_enumeration_5() {
    let src = r#"<** @description : Repeat modes for playback. **>
	enumeration RepeatMode {
		MODE_REPEAT_NONE   = 0
		MODE_REPEAT_SINGLE = 1,
		MODE_REPEAT_ALL    = 2
	}"#;
    let result = shared(src, enumeration::<BasicContext>, Rules::enumeration); 
    assert_eq!(result, (true, src.len() as u32));
}