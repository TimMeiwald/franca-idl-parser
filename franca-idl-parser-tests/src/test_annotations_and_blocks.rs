use crate::shared;
use franca_idl_parser::{annotation, annotation_block, BasicContext, Rules};
#[test]
fn test_annotation_1() {
    let src = "@input: This be an annotation ";
    let result = shared(src, annotation::<BasicContext>, Rules::annotation);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_annotation_2() {
    let src = "@input : This be an annotation \n";
    let result = shared(src, annotation::<BasicContext>, Rules::annotation);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_annotation_3() {
    let src = "@input : This be an annotation \n@input : This be an annotat";
    let result = shared(src, annotation::<BasicContext>, Rules::annotation);
    assert_eq!(result, (true, 32));
}
#[test]
fn test_annotation_block_1() {
    let src = "<** @input : This be an annotation   **>";
    let result = shared(src, annotation_block::<BasicContext>, Rules::annotation_block);
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_annotation_block_2() {
    let src = "<** @input : This be an annotation  \n Yet another bit of text **>";
    let result = shared(src, annotation_block::<BasicContext>, Rules::annotation_block);
    assert_eq!(result, (true, src.len() as u32));
}


#[test]
fn test_annotation_block_3() {
    let src = "<** @input : This be an annotation  \n @anotherinput: Yet another bit of text **>";
    let result = shared(src, annotation_block::<BasicContext>, Rules::annotation_block);
    assert_eq!(result, (true, src.len() as u32));
}
#[test]
fn test_annotation_block_4() {
    let src = "<** @description  : Bluetooth Manager interface.

    As this is an example interface only, it doesn't contain any
    more documentation. It is just a very simple interface definition.
    
    @source-alias : derived from org.blueman.Applet **>";
    let result = shared(src, annotation_block::<BasicContext>, Rules::annotation_block);
    assert_eq!(result, (true, src.len() as u32));
}

