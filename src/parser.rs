#![allow(non_camel_case_types)] // Generated Code kinda annoying to deal with so w/e
#![allow(unused_variables)] // Generated Code also, since everything passes stuff
#![allow(unused_imports)] // Generated Code also, since everything passes stuff
use crate::*;
use std::cell::RefCell;
#[allow(dead_code)]
pub fn grammar<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::package, context, package);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn ws_sole<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b' ');
	let closure_2 = _terminal(b'\t');
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b'\r');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	closure_5(parent, source, position)

} #[allow(dead_code)]
pub fn ws<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| ws_sole(parent, context, source, position);
	let closure_2 = _zero_or_more(&closure_1);
	closure_2(parent, source, position)

} #[allow(dead_code)]
pub fn wsn_sole<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b' ');
	let closure_2 = _terminal(b'\t');
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b'\r');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _terminal(b'\n');
	let closure_7 = _ordered_choice(&closure_5, &closure_6);
	closure_7(parent, source, position)

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
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_5, &closure_7);
	let closure_9 = _terminal(b'\n');
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn uri_string<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::letter, context, letter);
	let closure_2 = _one_or_more(&closure_1);
	let closure_3 = _terminal(b'.');
	let closure_4 = _var_name(Rules::letter, context, letter);
	let closure_5 = _one_or_more(&closure_4);
	let closure_6 = _sequence(&closure_3, &closure_5);
	let closure_7 = _subexpression(&closure_6);
	let closure_8 = _zero_or_more(&closure_7);
	let closure_9 = _sequence(&closure_2, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn letter<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _ordered_choice_match_range(65, 90);
	let closure_2 = _ordered_choice_match_range(97, 122);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	closure_3(parent, source, position)

} #[allow(dead_code)]
pub fn variable_str<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	//  Common to most PLS 
	let closure_1 = _ordered_choice_match_range(97, 122);
	let closure_2 = _ordered_choice_match_range(65, 90);
	let closure_3 = _ordered_choice(&closure_1, &closure_2);
	let closure_4 = _terminal(b'_');
	let closure_5 = _ordered_choice(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _ordered_choice_match_range(97, 122);
	let closure_8 = _ordered_choice_match_range(65, 90);
	let closure_9 = _ordered_choice(&closure_7, &closure_8);
	let closure_10 = _terminal(b'_');
	let closure_11 = _ordered_choice(&closure_9, &closure_10);
	let closure_12 = _ordered_choice_match_range(48, 57);
	let closure_13 = _ordered_choice(&closure_11, &closure_12);
	let closure_14 = _subexpression(&closure_13);
	let closure_15 = _subexpression(&closure_14);
	let closure_16 = _sequence(&closure_6, &closure_15);
	closure_16(parent, source, position)

} #[allow(dead_code)]
pub fn ascii<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _ordered_choice_match_range(0, 255);
	closure_1(parent, source, position)

} #[allow(dead_code)]
pub fn multiline_comment<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'<',b'*',b'*']);
	let closure_2 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = _var_name(Rules::ascii, context, ascii);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	let closure_9 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn comment<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'/',b'/']);
	let closure_2 = _terminal(b'\n');
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = _var_name(Rules::ascii, context, ascii);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _zero_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	closure_8(parent, source, position)

} #[allow(dead_code)]
pub fn annotation_block<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'<',b'*',b'*']);
	let closure_2 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_3 = _not_predicate(&closure_2);
	let closure_4 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _subexpression(&closure_5);
	let closure_7 = _one_or_more(&closure_6);
	let closure_8 = _sequence(&closure_1, &closure_7);
	let closure_9 = _string_terminal_opt_ascii(&[b'*',b'*',b'>']);
	let closure_10 = _sequence(&closure_8, &closure_9);
	closure_10(parent, source, position)

} #[allow(dead_code)]
pub fn annotation<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _terminal(b'@');
	let closure_2 = _var_name(Rules::letter, context, letter);
	let closure_3 = _one_or_more(&closure_2);
	let closure_4 = _sequence(&closure_1, &closure_3);
	let closure_5 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_6 = _zero_or_more(&closure_5);
	let closure_7 = _sequence(&closure_4, &closure_6);
	let closure_8 = _terminal(b':');
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _terminal(b'@');
	let closure_11 = _not_predicate(&closure_10);
	let closure_12 = _var_name(Rules::ascii, context, ascii);
	let closure_13 = _zero_or_more(&closure_12);
	let closure_14 = _sequence(&closure_11, &closure_13);
	let closure_15 = _subexpression(&closure_14);
	let closure_16 = _sequence(&closure_9, &closure_15);
	closure_16(parent, source, position)

} #[allow(dead_code)]
pub fn interface<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'i',b'n',b't',b'e',b'r',b'f',b'a',b'c',b'e']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::s_interface, context, s_interface);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _terminal(b'{');
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _var_name(Rules::interface_container, context, interface_container);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = _terminal(b'}');
	let closure_17 = _sequence(&closure_15, &closure_16);
	closure_17(parent, source, position)

} #[allow(dead_code)]
pub fn interface_container<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::version, context, version);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::method, context, method);
	let closure_5 = _var_name(Rules::broadcast, context, broadcast);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = _var_name(Rules::typedef, context, typedef);
	let closure_8 = _sequence(&closure_6, &closure_7);
	let closure_9 = _subexpression(&closure_8);
	let closure_10 = _one_or_more(&closure_9);
	let closure_11 = _sequence(&closure_3, &closure_10);
	closure_11(parent, source, position)

} #[allow(dead_code)]
pub fn version<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::version, context, version);
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
pub fn s_type<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| variable_str(parent, context, source, position);
	let closure_2 = _string_terminal_opt_ascii(&[b'[',b']']);
	let closure_3 = _subexpression(&closure_2);
	let closure_4 = _optional(&closure_3);
	let closure_5 = _sequence(&closure_1, &closure_4);
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
pub fn declaration<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::s_type, context, s_type);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _zero_or_more(&closure_2);
	let closure_4 = _sequence(&closure_1, &closure_3);
	let closure_5 = _var_name(Rules::s_variable, context, s_variable);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_8 = _zero_or_more(&closure_7);
	let closure_9 = _sequence(&closure_6, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn attribute<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _string_terminal_opt_ascii(&[b'a',b't',b't',b'r',b'i',b'b',b'u',b't',b'e']);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _var_name(Rules::type, context, type);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::s_variable, context, s_variable);
	let closure_9 = _sequence(&closure_7, &closure_8);
	closure_9(parent, source, position)

} #[allow(dead_code)]
pub fn enumeration<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _string_terminal_opt_ascii(&[b'e',b'n',b'u',b'm',b'e',b'r',b'a',b't',b'i',b'o',b'n']);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::s_type, context, s_type);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _terminal(b'{');
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _var_name(Rules::enum_value, context, enum_value);
	let closure_15 = _one_or_more(&closure_14);
	let closure_16 = _sequence(&closure_13, &closure_15);
	let closure_17 = _terminal(b'}');
	let closure_18 = _sequence(&closure_16, &closure_17);
	closure_18(parent, source, position)

} #[allow(dead_code)]
pub fn enum_value<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::s_variable, context, s_variable);
	let closure_2 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _terminal(b'=');
	let closure_5 = _var_name(Rules::integer, context, integer);
	let closure_6 = _sequence(&closure_4, &closure_5);
	let closure_7 = _subexpression(&closure_6);
	let closure_8 = _optional(&closure_7);
	let closure_9 = _sequence(&closure_3, &closure_8);
	let closure_10 = _terminal(b'\n');
	let closure_11 = _sequence(&closure_9, &closure_10);
	closure_11(parent, source, position)

} #[allow(dead_code)]
pub fn struct<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_2 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = _string_terminal_opt_ascii(&[b's',b't',b'r',b'u',b'c',b't']);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = _var_name(Rules::s_type, context, s_type);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = _terminal(b'{');
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_15 = _var_name(Rules::declaration, context, declaration);
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = _subexpression(&closure_18);
	let closure_20 = _one_or_more(&closure_19);
	let closure_21 = _sequence(&closure_13, &closure_20);
	let closure_22 = _terminal(b'}');
	let closure_23 = _sequence(&closure_21, &closure_22);
	closure_23(parent, source, position)

} #[allow(dead_code)]
pub fn broadcast<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _string_terminal_opt_ascii(&[b'b',b'r',b'o',b'a',b'd',b'c',b'a',b's',b't']);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _terminal(b'{');
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _var_name(Rules::out, context, out);
	let closure_15 = _optional(&closure_14);
	let closure_16 = _sequence(&closure_13, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = _terminal(b'}');
	let closure_20 = _sequence(&closure_18, &closure_19);
	let closure_21 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_22 = _sequence(&closure_20, &closure_21);
	closure_22(parent, source, position)

} #[allow(dead_code)]
pub fn method<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _string_terminal_opt_ascii(&[b'm',b'e',b't',b'h',b'o',b'd']);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _terminal(b'{');
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _var_name(Rules::in, context, in);
	let closure_15 = _optional(&closure_14);
	let closure_16 = _sequence(&closure_13, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	let closure_19 = _var_name(Rules::out, context, out);
	let closure_20 = _optional(&closure_19);
	let closure_21 = _sequence(&closure_18, &closure_20);
	let closure_22 = _terminal(b'}');
	let closure_23 = _sequence(&closure_21, &closure_22);
	closure_23(parent, source, position)

} #[allow(dead_code)]
pub fn in<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _string_terminal_opt_ascii(&[b'i',b'n']);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _terminal(b'{');
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _var_name(Rules::declaration, context, declaration);
	let closure_11 = _zero_or_more(&closure_10);
	let closure_12 = _sequence(&closure_9, &closure_11);
	let closure_13 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = _terminal(b'}');
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	closure_18(parent, source, position)

} #[allow(dead_code)]
pub fn out<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _string_terminal_opt_ascii(&[b'o',b'u',b't']);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _terminal(b'{');
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _var_name(Rules::declaration, context, declaration);
	let closure_11 = _zero_or_more(&closure_10);
	let closure_12 = _sequence(&closure_9, &closure_11);
	let closure_13 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_14 = _sequence(&closure_12, &closure_13);
	let closure_15 = _terminal(b'}');
	let closure_16 = _sequence(&closure_14, &closure_15);
	let closure_17 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_18 = _sequence(&closure_16, &closure_17);
	closure_18(parent, source, position)

} #[allow(dead_code)]
pub fn typedef<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {

	let closure_1 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_2 = _var_name(Rules::annotation_block, context, annotation_block);
	let closure_3 = _sequence(&closure_1, &closure_2);
	let closure_4 = move |parent: Key, source: &Source, position: u32| wsn(parent, context, source, position);
	let closure_5 = _sequence(&closure_3, &closure_4);
	let closure_6 = _string_terminal_opt_ascii(&[b't',b'y',b'p',b'e',b'd',b'e',b'f']);
	let closure_7 = _sequence(&closure_5, &closure_6);
	let closure_8 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_9 = _sequence(&closure_7, &closure_8);
	let closure_10 = _var_name(Rules::s_type, context, s_type);
	let closure_11 = _sequence(&closure_9, &closure_10);
	let closure_12 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_13 = _sequence(&closure_11, &closure_12);
	let closure_14 = _string_terminal_opt_ascii(&[b'i',b's']);
	let closure_15 = _sequence(&closure_13, &closure_14);
	let closure_16 = move |parent: Key, source: &Source, position: u32| ws(parent, context, source, position);
	let closure_17 = _sequence(&closure_15, &closure_16);
	let closure_18 = _var_name(Rules::s_type, context, s_type);
	let closure_19 = _sequence(&closure_17, &closure_18);
	closure_19(parent, source, position)

} #[allow(dead_code)]
pub fn psm<T: Context>(parent: Key, context: &RefCell<T>, source: &Source, position: u32) -> (bool, u32) {
	// Program State Machine
	let closure_1 = _string_terminal_opt_ascii(&[b'W',b'A',b'A',b'A',b'A',b'G',b'H',b' ',b'-',b' ',b'N',b'o',b't',b' ',b'Y',b'e',b't',b' ',b'I',b'm',b'p',b'l',b'e',b'm',b'e',b'n',b't',b'e',b'd']);
	closure_1(parent, source, position)

} 