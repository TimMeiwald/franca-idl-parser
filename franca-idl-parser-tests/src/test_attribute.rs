use crate::shared;
use franca_idl_parser::{attribute, annotation_block, BasicContext, Rules};

// 	attribute Playlist currentPlaylist

// 	attribute TrackId currentTrack
	
// 	attribute Duration remainingTrack
	
// 	attribute Duration remainingAll
	
// 	attribute RepeatMode mode

#[test]
fn test_attribute_1() {
    let src = "attribute Playlist currentPlaylist";
    let result = shared(src, attribute::<BasicContext>, Rules::attribute);
    assert_eq!(result, (true, src.len() as u32));
}