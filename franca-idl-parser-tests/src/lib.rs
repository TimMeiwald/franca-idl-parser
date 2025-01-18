mod test_annotations_and_blocks;
mod test_attribute;
mod test_version;
mod test_variable_string;
use franca_idl_parser::*;

use std::cell::RefCell;

fn shared<'a>(
    input: &str,
    func: for<'c> fn(Key, &RefCell<BasicContext>, &Source<'c>, u32) -> (bool, u32),
    rule: Rules,
) -> (bool, u32) {
    let string = input.to_string();
    let src_len = string.len() as u32;
    let source = Source::new(&string);
    let position: u32 = 0;
    let result: (bool, u32);
    let context = RefCell::new(BasicContext::new(src_len as usize, RULES_SIZE as usize));
    {
        let executor = _var_name(rule, &context, func);
        result = executor(Key(0), &source, position);
    }
    println!("Result: {:?}", result);
    //context.borrow().print_cache();
    //context.borrow().print_publisher();
    context.borrow().print_node(Key(0));

    result
}


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

