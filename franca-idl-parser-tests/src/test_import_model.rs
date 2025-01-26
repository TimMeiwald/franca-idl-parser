use crate::shared;
use franca_idl_parser::{annotation_block, import_model, variable_str, BasicContext, Rules};


#[test]
fn test_import_model_1() {
    let src = r#"import model "csm_t.fidl""#;
    let result = shared(src, import_model::<BasicContext>, Rules::import_model); 
    assert_eq!(result, (true, src.len() as u32));
}
