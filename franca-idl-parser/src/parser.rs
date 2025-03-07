#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn grammar<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _var_name(Rules::package, context, package);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _var_name(Rules::import, context, import);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_5, &closure_7);
	let closure_9 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = _var_name(Rules::interface, context, interface);
	let closure_12 = _var_name(Rules::type_collection, context, type_collection);
	let closure_13 = _ordered_choice(&closure_11, &closure_12);
	let closure_14 = _subexpression(&closure_13);
	let closure_15 = _zero_or_more(&closure_14);
	let closure_16 = _sequence(&closure_10, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	closure_18(parent, source, position)

} #[allow(dead_code)]
pub fn ws_sole<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::multiline_comment, context, multiline_comment);
	let closure_2 = _var_name(Rules::comment, context, comment);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b' ');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _terminal(b'\t');
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	let closure_8 = _terminal(b'\r');
	let closure_9 = _ordered_choice(&closure_7, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn ws<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| ws_sole(parent, context, source, position);
	let closure_2 = _zero_or_more(&closure_1);
	closure_2(parent, source, position)

} #[allow(dead_code)]
pub fn wsn_sole<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::multiline_comment, context, multiline_comment);
	let closure_2 = _var_name(Rules::comment, context, comment);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b' ');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _terminal(b'\t');
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	let closure_8 = _terminal(b'\r');
	let closure_9 = _ordered_choice(&closure_7, &closure_8);
	let closure_10 = _terminal(b'\n');
	let closure_11 = _ordered_choice(&closure_9, &closure_10);
	closure_11(parent, source, position)

} #[allow(dead_code)]
pub fn wsn<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn_sole(parent, context, source, position);
	let closure_2 = _zero_or_more(&closure_1);
	closure_2(parent, source, position)

} #[allow(dead_code)]
pub fn package<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	// Describes the package import
	let closure_1 = _string_terminal_opt_ascii(&[b'p',b'a',b'c',b'k',b'a',b'g',b'e']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::uri_string, context, uri_string);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _terminal(b'\n');
	let closure_9 = _sequence(&closure_7, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn num<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _ordered_choice_match_range(48, 57);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn imported_uri_string<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::uri_string, context, uri_string);
	let closure_2 = _string_terminal_opt_ascii(&[b'.',b'*']);
	let closure_3 = _optional(&closure_2);
	let closure_4 = _sequence(&closure_1, &closure_3);
	closure_4(parent, source, position)

} #[allow(dead_code)]
pub fn uri_string<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| letter(parent, context, source, position);
	let closure_2 = _var_name(Rules::num, context, num);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b'-');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _terminal(b'_');
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	let closure_8 = _subexpression(&closure_7);
	let closure_9 = _one_or_more(&closure_8);
	let closure_10 = _terminal(b'.');
	let closure_11 = move |parent: Key, source: &Source, position: u32| letter(parent, context, source, position);
	let closure_12 = _var_name(Rules::num, context, num);
	let closure_13 = _ordered_choice(&closure_11, &closure_12);
	let closure_14 = _terminal(b'-');
	let closure_15 = _ordered_choice(&closure_13, &closure_14);
	let closure_16 = _terminal(b'_');
	let closure_17 = _ordered_choice(&closure_15, &closure_16);
	let closure_18 = _subexpression(&closure_17);
	let closure_19 = _one_or_more(&closure_18);
	let closure_20 = _sequence(&closure_10, &closure_19);
	let closure_21 = _subexpression(&closure_20);
	let closure_22 = _zero_or_more(&closure_21);
	let closure_23 = _sequence(&closure_9, &closure_22);
	closure_23(parent, source, position)

} #[allow(dead_code)]
pub fn letter<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _ordered_choice_match_range(65, 90);
	let closure_2 = _ordered_choice_match_range(97, 122);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn variable_str<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	//  Common to most PLS 
	let closure_1 = _terminal(b'^');
	let closure_2 = _optional(&closure_1);
	let closure_3 = _ordered_choice_match_range(97, 122);
	let closure_4 = _ordered_choice_match_range(65, 90);
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _terminal(b'_');
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	let closure_8 = _subexpression(&closure_7);
	let closure_9 = _sequence(&closure_2, &closure_8);
	let closure_10 = _ordered_choice_match_range(97, 122);
	let closure_11 = _ordered_choice_match_range(65, 90);
	let closure_12 = _ordered_choice(&closure_10, &closure_11);
	let closure_13 = _terminal(b'_');
	let closure_14 = _ordered_choice(&closure_12, &closure_13);
	let closure_15 = _ordered_choice_match_range(48, 57);
	let closure_16 = _ordered_choice(&closure_14, &closure_15);
	let closure_17 = _subexpression(&closure_16);
	let closure_18 = _subexpression(&closure_17);
	let closure_19 = _zero_or_more(&closure_18);
	let closure_20 = _sequence(&closure_9, &closure_19);
	closure_20(parent, source, position)

} #[allow(dead_code)]
pub fn file_name<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b'"');
	let closure_2 = _terminal(b'"');
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	let closure_9 = _terminal(b'"');
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn import_namespace<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'i',b'm',b'p',b'o',b'r',b't']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::imported_uri_string, context, imported_uri_string);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _string_terminal_opt_ascii(&[b'f',b'r',b'o',b'm']);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _var_name(Rules::file_name, context, file_name);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _sequence(&closure_13, &closure_14);
	closure_15(parent, source, position)

} #[allow(dead_code)]
pub fn import_model<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'i',b'm',b'p',b'o',b'r',b't']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _string_terminal_opt_ascii(&[b'm',b'o',b'd',b'e',b'l']);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::file_name, context, file_name);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	closure_11(parent, source, position)

} #[allow(dead_code)]
pub fn import<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::import_model, context, import_model);
	let closure_2 = _var_name(Rules::import_namespace, context, import_namespace);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn ascii<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _ordered_choice_match_range(0, 255);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn multiline_comment<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'/',b'*']);
	let closure_2 = _string_terminal_opt_ascii(&[b'*',b'/']);
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	let closure_9 = _string_terminal_opt_ascii(&[b'*',b'/']);
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn comment<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'/',b'/']);
	let closure_2 = _terminal(b'\n');
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	let closure_9 = _terminal(b'\n');
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn annotation_block<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'<',b'*',b'*']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_5 = _not_predicate(&closure_4);
	let closure_6 = _var_name(Rules::annotation, context, annotation);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _subexpression(&closure_9);
	let closure_11 = _one_or_more(&closure_10);
	let closure_12 = _sequence(&closure_3, &closure_11);
	let closure_13 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_16 = _sequence(&closure_14, &closure_15);
	closure_16(parent, source, position)

} #[allow(dead_code)]
pub fn annotation<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b'@');
	let closure_2 = _var_name(Rules::annotation_name, context, annotation_name);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _terminal(b':');
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::annotation_content, context, annotation_content);
	let closure_9 = _sequence(&closure_7, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn annotation_content<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b'@');
	let closure_2 = _not_predicate(&closure_1);
	let closure_3 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_4 = _not_predicate(&closure_3);
	let closure_5 = _sequence(&closure_2, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ascii(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _subexpression(&closure_7);
	let closure_9 = _zero_or_more(&closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn annotation_name<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| annotation_name_char(parent, context, source, position);
	let closure_2 = _one_or_more(&closure_1);
	closure_2(parent, source, position)

} #[allow(dead_code)]
pub fn annotation_name_char<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| letter(parent, context, source, position);
	let closure_2 = _terminal(b'-');
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn extends<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'e',b'x',b't',b'e',b'n',b'd',b's']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::s_type, context, s_type);
	let closure_5 = _sequence(&closure_3, &closure_4);
	closure_5(parent, source, position)

} #[allow(dead_code)]
pub fn interface<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'i',b'n',b't',b'e',b'r',b'f',b'a',b'c',b'e']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_interface, context, s_interface);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _var_name(Rules::extends, context, extends);
	let closure_14 = _optional(&closure_13);
	let closure_15 = _sequence(&closure_12, &closure_14);
	let closure_16 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = _terminal(b'{');
	let closure_19 = _sequence(&closure_17, &closure_18);
	let closure_20 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_21 = _sequence(&closure_19, &closure_20);
	let closure_22 = _var_name(Rules::interface_container, context, interface_container);
	let closure_23 = _sequence(&closure_21, &closure_22);
	let closure_24 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_25 = _sequence(&closure_23, &closure_24);
	let closure_26 = _terminal(b'}');
	let closure_27 = _sequence(&closure_25, &closure_26);
	let closure_28 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_29 = _sequence(&closure_27, &closure_28);
	closure_29(parent, source, position)

} #[allow(dead_code)]
pub fn interface_container<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::version, context, version);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _var_name(Rules::method, context, method);
	let closure_6 = _var_name(Rules::broadcast, context, broadcast);
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::typedef, context, typedef);
	let closure_9 = _ordered_choice(&closure_7, &closure_8);
	let closure_10 = _var_name(Rules::explicit_array_def, context, explicit_array_def);
	let closure_11 = _ordered_choice(&closure_9, &closure_10);
	let closure_12 = _var_name(Rules::structure, context, structure);
	let closure_13 = _ordered_choice(&closure_11, &closure_12);
	let closure_14 = _var_name(Rules::attribute, context, attribute);
	let closure_15 = _ordered_choice(&closure_13, &closure_14);
	let closure_16 = _var_name(Rules::enumeration, context, enumeration);
	let closure_17 = _ordered_choice(&closure_15, &closure_16);
	let closure_18 = _var_name(Rules::map, context, map);
	let closure_19 = _ordered_choice(&closure_17, &closure_18);
	let closure_20 = _var_name(Rules::union, context, union);
	let closure_21 = _ordered_choice(&closure_19, &closure_20);
	let closure_22 = _var_name(Rules::comment, context, comment);
	let closure_23 = _ordered_choice(&closure_21, &closure_22);
	let closure_24 = _var_name(Rules::multiline_comment, context, multiline_comment);
	let closure_25 = _ordered_choice(&closure_23, &closure_24);
	let closure_26 = _subexpression(&closure_25);
	let closure_27 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_28 = _sequence(&closure_26, &closure_27);
	let closure_29 = _subexpression(&closure_28);
	let closure_30 = _zero_or_more(&closure_29);
	let closure_31 = _sequence(&closure_4, &closure_30);
	closure_31(parent, source, position)

} #[allow(dead_code)]
pub fn version<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'v',b'e',b'r',b's',b'i',b'o',b'n']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _terminal(b'{');
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::major, context, major);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _var_name(Rules::minor, context, minor);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = _terminal(b'}');
	let closure_17 = _sequence(&closure_15, &closure_16);
	closure_17(parent, source, position)

} #[allow(dead_code)]
pub fn integer<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	// Could prevent leading zeros but whatever
	let closure_1 = _ordered_choice_match_range(48, 57);
	let closure_2 = _one_or_more(&closure_1);
	closure_2(parent, source, position)

} #[allow(dead_code)]
pub fn major<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'm',b'a',b'j',b'o',b'r']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::integer, context, integer);
	let closure_5 = _sequence(&closure_3, &closure_4);
	closure_5(parent, source, position)

} #[allow(dead_code)]
pub fn minor<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'm',b'i',b'n',b'o',b'r']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::integer, context, integer);
	let closure_5 = _sequence(&closure_3, &closure_4);
	closure_5(parent, source, position)

} #[allow(dead_code)]
pub fn mininteger<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'm',b'a',b'x',b'I',b'n',b't']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn maxinteger<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'm',b'i',b'n',b'I',b'n',b't']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn integer_range<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b'(');
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::integer, context, integer);
	let closure_5 = _var_name(Rules::minInteger, context, mininteger);
	let closure_6 = _ordered_choice(&closure_4, &closure_5);
	let closure_7 = _var_name(Rules::maxInteger, context, maxinteger);
	let closure_8 = _ordered_choice(&closure_6, &closure_7);
	let closure_9 = _subexpression(&closure_8);
	let closure_10 = _sequence(&closure_3, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _terminal(b',');
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = _var_name(Rules::integer, context, integer);
	let closure_18 = _var_name(Rules::minInteger, context, mininteger);
	let closure_19 = _ordered_choice(&closure_17, &closure_18);
	let closure_20 = _var_name(Rules::maxInteger, context, maxinteger);
	let closure_21 = _ordered_choice(&closure_19, &closure_20);
	let closure_22 = _subexpression(&closure_21);
	let closure_23 = _sequence(&closure_16, &closure_22);
	let closure_24 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_25 = _sequence(&closure_23, &closure_24);
	let closure_26 = _terminal(b')');
	let closure_27 = _sequence(&closure_25, &closure_26);
	let closure_28 = _subexpression(&closure_27);
	closure_28(parent, source, position)

} #[allow(dead_code)]
pub fn s_type_integer_range<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	let closure_2 = _var_name(Rules::integer_range, context, integer_range);
	let closure_3 = _sequence(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn s_type_array<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::s_type, context, s_type);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _terminal(b'[');
	let closure_5 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = _terminal(b']');
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _subexpression(&closure_8);
	let closure_10 = _sequence(&closure_3, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn s_type<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	let closure_2 = _terminal(b'.');
	let closure_3 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _subexpression(&closure_4);
	let closure_6 = _zero_or_more(&closure_5);
	let closure_7 = _sequence(&closure_1, &closure_6);
	closure_7(parent, source, position)

} #[allow(dead_code)]
pub fn s_typeof<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::s_type_array, context, s_type_array);
	let closure_2 = _var_name(Rules::s_type_integer_range, context, s_type_integer_range);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::s_type, context, s_type);
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	closure_5(parent, source, position)

} #[allow(dead_code)]
pub fn s_variable<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn s_interface<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn s_type_collection<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn type_collection<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b't',b'y',b'p',b'e',b'C',b'o',b'l',b'l',b'e',b'c',b't',b'i',b'o',b'n']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_variable, context, s_variable);
	let closure_10 = _optional(&closure_9);
	let closure_11 = _sequence(&closure_8, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _terminal(b'{');
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = _var_name(Rules::version, context, version);
	let closure_19 = _optional(&closure_18);
	let closure_20 = _sequence(&closure_17, &closure_19);
	let closure_21 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_22 = _var_name(Rules::const_def, context, const_def);
	let closure_23 = _var_name(Rules::explicit_array_def, context, explicit_array_def);
	let closure_24 = _ordered_choice(&closure_22, &closure_23);
	let closure_25 = _var_name(Rules::typedef, context, typedef);
	let closure_26 = _ordered_choice(&closure_24, &closure_25);
	let closure_27 = _var_name(Rules::structure, context, structure);
	let closure_28 = _ordered_choice(&closure_26, &closure_27);
	let closure_29 = _var_name(Rules::enumeration, context, enumeration);
	let closure_30 = _ordered_choice(&closure_28, &closure_29);
	let closure_31 = _var_name(Rules::union, context, union);
	let closure_32 = _ordered_choice(&closure_30, &closure_31);
	let closure_33 = _var_name(Rules::map, context, map);
	let closure_34 = _ordered_choice(&closure_32, &closure_33);
	let closure_35 = _subexpression(&closure_34);
	let closure_36 = _sequence(&closure_21, &closure_35);
	let closure_37 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_38 = _sequence(&closure_36, &closure_37);
	let closure_39 = _subexpression(&closure_38);
	let closure_40 = _zero_or_more(&closure_39);
	let closure_41 = _sequence(&closure_20, &closure_40);
	let closure_42 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_43 = _sequence(&closure_41, &closure_42);
	let closure_44 = _terminal(b'}');
	let closure_45 = _sequence(&closure_43, &closure_44);
	let closure_46 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_47 = _sequence(&closure_45, &closure_46);
	closure_47(parent, source, position)

} #[allow(dead_code)]
pub fn declaration<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws_sole(parent, context, source, position);
	let closure_8 = _one_or_more(&closure_7);
	let closure_9 = _sequence(&closure_6, &closure_8);
	let closure_10 = _var_name(Rules::s_variable, context, s_variable);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_13 = _sequence(&closure_11, &closure_12);
	closure_13(parent, source, position)

} #[allow(dead_code)]
pub fn attribute_meta_information<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'r',b'e',b'a',b'd',b'o',b'n',b'l',b'y']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn bool<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b't',b'r',b'u',b'e']);
	let closure_2 = _string_terminal_opt_ascii(&[b'f',b'a',b'l',b's',b'e']);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn const_def<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'c',b'o',b'n',b's',b't']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_type, context, s_type);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _var_name(Rules::s_variable, context, s_variable);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = _terminal(b'=');
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_20 = _sequence(&closure_18, &closure_19);
	let closure_21 = _var_name(Rules::integer, context, integer);
	let closure_22 = _var_name(Rules::bool, context, bool);
	let closure_23 = _ordered_choice(&closure_21, &closure_22);
	let closure_24 = _subexpression(&closure_23);
	let closure_25 = _sequence(&closure_20, &closure_24);
	let closure_26 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_27 = _sequence(&closure_25, &closure_26);
	let closure_28 = _terminal(b',');
	let closure_29 = _optional(&closure_28);
	let closure_30 = _sequence(&closure_27, &closure_29);
	let closure_31 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_32 = _sequence(&closure_30, &closure_31);
	closure_32(parent, source, position)

} #[allow(dead_code)]
pub fn attribute<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'a',b't',b't',b'r',b'i',b'b',b'u',b't',b'e']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws_sole(parent, context, source, position);
	let closure_8 = _one_or_more(&closure_7);
	let closure_9 = _sequence(&closure_6, &closure_8);
	let closure_10 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| ws_sole(parent, context, source, position);
	let closure_13 = _one_or_more(&closure_12);
	let closure_14 = _sequence(&closure_11, &closure_13);
	let closure_15 = _var_name(Rules::s_variable, context, s_variable);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = _var_name(Rules::attribute_meta_information, context, attribute_meta_information);
	let closure_20 = _optional(&closure_19);
	let closure_21 = _sequence(&closure_18, &closure_20);
	closure_21(parent, source, position)

} #[allow(dead_code)]
pub fn enumeration<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'e',b'n',b'u',b'm',b'e',b'r',b'a',b't',b'i',b'o',b'n']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _terminal(b'{');
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_16 = _var_name(Rules::enum_value, context, enum_value);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_19 = _sequence(&closure_17, &closure_18);
	let closure_20 = _subexpression(&closure_19);
	let closure_21 = _one_or_more(&closure_20);
	let closure_22 = _sequence(&closure_14, &closure_21);
	let closure_23 = _terminal(b'}');
	let closure_24 = _sequence(&closure_22, &closure_23);
	closure_24(parent, source, position)

} #[allow(dead_code)]
pub fn enum_value<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _var_name(Rules::s_variable, context, s_variable);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _terminal(b'=');
	let closure_10 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _var_name(Rules::integer, context, integer);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _subexpression(&closure_13);
	let closure_15 = _optional(&closure_14);
	let closure_16 = _sequence(&closure_8, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = _terminal(b',');
	let closure_20 = _optional(&closure_19);
	let closure_21 = _sequence(&closure_18, &closure_20);
	let closure_22 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_23 = _sequence(&closure_21, &closure_22);
	closure_23(parent, source, position)

} #[allow(dead_code)]
pub fn union<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'u',b'n',b'i',b'o',b'n']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _var_name(Rules::extends, context, extends);
	let closure_14 = _optional(&closure_13);
	let closure_15 = _sequence(&closure_12, &closure_14);
	let closure_16 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = _terminal(b'{');
	let closure_19 = _sequence(&closure_17, &closure_18);
	let closure_20 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_21 = _sequence(&closure_19, &closure_20);
	let closure_22 = _var_name(Rules::declaration, context, declaration);
	let closure_23 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_24 = _sequence(&closure_22, &closure_23);
	let closure_25 = _subexpression(&closure_24);
	let closure_26 = _zero_or_more(&closure_25);
	let closure_27 = _sequence(&closure_21, &closure_26);
	let closure_28 = _terminal(b'}');
	let closure_29 = _sequence(&closure_27, &closure_28);
	closure_29(parent, source, position)

} #[allow(dead_code)]
pub fn map<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'm',b'a',b'p']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _terminal(b'{');
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_16 = _var_name(Rules::s_type, context, s_type);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_19 = _sequence(&closure_17, &closure_18);
	let closure_20 = _string_terminal_opt_ascii(&[b't',b'o']);
	let closure_21 = _sequence(&closure_19, &closure_20);
	let closure_22 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_23 = _sequence(&closure_21, &closure_22);
	let closure_24 = _var_name(Rules::s_type, context, s_type);
	let closure_25 = _sequence(&closure_23, &closure_24);
	let closure_26 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_27 = _sequence(&closure_25, &closure_26);
	let closure_28 = _subexpression(&closure_27);
	let closure_29 = _one_or_more(&closure_28);
	let closure_30 = _sequence(&closure_14, &closure_29);
	let closure_31 = _terminal(b'}');
	let closure_32 = _sequence(&closure_30, &closure_31);
	closure_32(parent, source, position)

} #[allow(dead_code)]
pub fn polymorphic<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'p',b'o',b'l',b'y',b'm',b'o',b'r',b'p',b'h',b'i',b'c']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn structure<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b's',b't',b'r',b'u',b'c',b't']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _var_name(Rules::polymorphic, context, polymorphic);
	let closure_14 = _var_name(Rules::extends, context, extends);
	let closure_15 = _ordered_choice(&closure_13, &closure_14);
	let closure_16 = _subexpression(&closure_15);
	let closure_17 = _optional(&closure_16);
	let closure_18 = _sequence(&closure_12, &closure_17);
	let closure_19 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_20 = _sequence(&closure_18, &closure_19);
	let closure_21 = _terminal(b'{');
	let closure_22 = _sequence(&closure_20, &closure_21);
	let closure_23 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_24 = _sequence(&closure_22, &closure_23);
	let closure_25 = _var_name(Rules::declaration, context, declaration);
	let closure_26 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_27 = _sequence(&closure_25, &closure_26);
	let closure_28 = _subexpression(&closure_27);
	let closure_29 = _zero_or_more(&closure_28);
	let closure_30 = _sequence(&closure_24, &closure_29);
	let closure_31 = _terminal(b'}');
	let closure_32 = _sequence(&closure_30, &closure_31);
	let closure_33 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_34 = _sequence(&closure_32, &closure_33);
	closure_34(parent, source, position)

} #[allow(dead_code)]
pub fn broadcast<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'b',b'r',b'o',b'a',b'd',b'c',b'a',b's',b't']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _terminal(b'{');
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = _var_name(Rules::out, context, out);
	let closure_18 = _optional(&closure_17);
	let closure_19 = _sequence(&closure_16, &closure_18);
	let closure_20 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_21 = _sequence(&closure_19, &closure_20);
	let closure_22 = _terminal(b'}');
	let closure_23 = _sequence(&closure_21, &closure_22);
	let closure_24 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_25 = _sequence(&closure_23, &closure_24);
	closure_25(parent, source, position)

} #[allow(dead_code)]
pub fn method<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'm',b'e',b't',b'h',b'o',b'd']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _var_name(Rules::method_attribute, context, method_attribute);
	let closure_14 = _optional(&closure_13);
	let closure_15 = _sequence(&closure_12, &closure_14);
	let closure_16 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = _terminal(b'{');
	let closure_19 = _sequence(&closure_17, &closure_18);
	let closure_20 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_21 = _sequence(&closure_19, &closure_20);
	let closure_22 = _var_name(Rules::method_in, context, method_in);
	let closure_23 = _optional(&closure_22);
	let closure_24 = _sequence(&closure_21, &closure_23);
	let closure_25 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_26 = _sequence(&closure_24, &closure_25);
	let closure_27 = _var_name(Rules::out, context, out);
	let closure_28 = _optional(&closure_27);
	let closure_29 = _sequence(&closure_26, &closure_28);
	let closure_30 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_31 = _sequence(&closure_29, &closure_30);
	let closure_32 = _terminal(b'}');
	let closure_33 = _sequence(&closure_31, &closure_32);
	let closure_34 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_35 = _sequence(&closure_33, &closure_34);
	closure_35(parent, source, position)

} #[allow(dead_code)]
pub fn method_attribute<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'f',b'i',b'r',b'e',b'A',b'n',b'd',b'F',b'o',b'r',b'g',b'e',b't']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn method_in<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'i',b'n']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _terminal(b'{');
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _var_name(Rules::declaration, context, declaration);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = _subexpression(&closure_15);
	let closure_17 = _one_or_more(&closure_16);
	let closure_18 = _sequence(&closure_10, &closure_17);
	let closure_19 = _terminal(b'}');
	let closure_20 = _sequence(&closure_18, &closure_19);
	let closure_21 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_22 = _sequence(&closure_20, &closure_21);
	closure_22(parent, source, position)

} #[allow(dead_code)]
pub fn out<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'o',b'u',b't']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _terminal(b'{');
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _var_name(Rules::declaration, context, declaration);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = _subexpression(&closure_15);
	let closure_17 = _one_or_more(&closure_16);
	let closure_18 = _sequence(&closure_10, &closure_17);
	let closure_19 = _terminal(b'}');
	let closure_20 = _sequence(&closure_18, &closure_19);
	let closure_21 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_22 = _sequence(&closure_20, &closure_21);
	closure_22(parent, source, position)

} #[allow(dead_code)]
pub fn typedef<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b't',b'y',b'p',b'e',b'd',b'e',b'f']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_type, context, s_type);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _string_terminal_opt_ascii(&[b'i',b's']);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = _var_name(Rules::s_typeof, context, s_typeof);
	let closure_18 = _sequence(&closure_16, &closure_17);
	closure_18(parent, source, position)

} #[allow(dead_code)]
pub fn psm<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	// Program State Machine
	let closure_1 = _string_terminal_opt_ascii(&[b'W',b'A',b'A',b'A',b'A',b'G',b'H',b' ',b'-',b' ',b'N',b'o',b't',b' ',b'Y',b'e',b't',b' ',b'I',b'm',b'p',b'l',b'e',b'm',b'e',b'n',b't',b'e',b'd']);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn explicit_array_def<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	//  No idea why this exists or what it even means in an interface tbh
	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = _optional(&closure_1);
	let closure_3 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_4 = _sequence(&closure_2, &closure_3);
	let closure_5 = _string_terminal_opt_ascii(&[b'a',b'r',b'r',b'a',b'y']);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _var_name(Rules::s_variable, context, s_variable);
	let closure_10 = _sequence(&closure_8, &closure_9);
	let closure_11 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_12 = _sequence(&closure_10, &closure_11);
	let closure_13 = _string_terminal_opt_ascii(&[b'o',b'f']);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = _var_name(Rules::s_type_integer_range, context, s_type_integer_range);
	let closure_18 = _var_name(Rules::s_type, context, s_type);
	let closure_19 = _ordered_choice(&closure_17, &closure_18);
	let closure_20 = _subexpression(&closure_19);
	let closure_21 = _sequence(&closure_16, &closure_20);
	let closure_22 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_23 = _sequence(&closure_21, &closure_22);
	closure_23(parent, source, position)

} 