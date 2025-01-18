#![allow(non_camel_case_types)] // Again due to generation -> Might solve eventually
use num_derive::FromPrimitive;
impl From<u32> for Rules {
    fn from(i: u32) -> Rules {
        let element = num::FromPrimitive::from_u32(i);
        match element {
            Some(rule) => rule,
            None => panic!("Not a valid Rule"),
        }
    }
}
#[allow(dead_code)]
pub static RULES_SIZE: u32 =  29;
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]

pub enum Rules {
	Grammar,
	annotation,
	annotation_block,
	annotation_content,
	annotation_name,
	attribute,
	broadcast,
	comment,
	declaration,
	enum_value,
	enumeration,
	integer,
	interface,
	interface_container,
	major,
	method,
	minor,
	multiline_comment,
	out,
	package,
	psm,
	s_interface,
	s_type,
	s_variable,
	structure,
	t_in,
	typedef,
	uri_string,
	version,

}