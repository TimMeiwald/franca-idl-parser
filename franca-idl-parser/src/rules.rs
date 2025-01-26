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
pub static RULES_SIZE: u32 =  52;
#[allow(clippy::upper_case_acronyms)] // Again due to generation -> Might solve eventually
#[derive(PartialEq, Eq, Hash, FromPrimitive, Clone, Copy, Debug, Ord, PartialOrd)]

pub enum Rules {
	Grammar,
	annotation,
	annotation_block,
	annotation_content,
	annotation_name,
	attribute,
	attribute_meta_information,
	bool,
	broadcast,
	comment,
	const_def,
	declaration,
	enum_value,
	enumeration,
	explicit_array_def,
	extends,
	file_name,
	import,
	import_model,
	import_namespace,
	imported_uri_string,
	integer,
	integer_range,
	interface,
	interface_container,
	major,
	map,
	maxInteger,
	method,
	method_attribute,
	method_in,
	minInteger,
	minor,
	multiline_comment,
	num,
	out,
	package,
	polymorphic,
	psm,
	s_interface,
	s_type,
	s_type_array,
	s_type_collection,
	s_type_integer_range,
	s_typeof,
	s_variable,
	structure,
	type_collection,
	typedef,
	union,
	uri_string,
	version,

}