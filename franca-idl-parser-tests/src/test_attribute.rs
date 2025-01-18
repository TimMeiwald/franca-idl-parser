use crate::shared;
use franca_idl_parser::{annotation_block, attribute, BasicContext, Rules};

// 	attribute Playlist currentPlaylist

// 	attribute TrackId currentTrack

// 	attribute Duration remainingTrack

// 	attribute Duration remainingAll

// 	attribute RepeatMode mode

#[test]
fn test_attribute_1() {
    let strs = vec![
        "attribute Playlist currentPlaylist",
        "attribute TrackId currentTrack",
        "attribute Duration remainingTrack",
        "attribute Duration remainingAll",
        "attribute RepeatMode mode",
    ];
    for str in strs{
        let result = shared(str, attribute::<BasicContext>, Rules::attribute);
        assert_eq!(result, (true, str.len() as u32));
    }
 
}

// <** @description: Playlist which is currently active. **>
// attribute Playlist currentPlaylist

// <** @description: Track which is currently playing. **>
// attribute TrackId currentTrack

// <** @description: Remaining duration of current track. **>
// attribute Duration remainingTrack

// <** @description: Remaining duration of current playlist. **>
// attribute Duration remainingAll


#[test]
fn test_attribute_2() {
    let strs = vec![
        "<** @description: Playlist which is currently active. **>\n\nattribute Playlist currentPlaylist",
        "<** @description: Track which is currently playing. **>\n\t attribute TrackId currentTrack",
        "<** @description: Remaining duration of current track. **>\r\n\tattribute Duration remainingTrack",
        "<** @description: Remaining duration of current track. **>attribute Duration remainingAll",
    ];
    for str in strs{
        let result = shared(str, attribute::<BasicContext>, Rules::attribute);
        assert_eq!(result, (true, str.len() as u32));
    }
 
}