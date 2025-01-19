use crate::shared;
use franca_idl_parser::{broadcast, BasicContext, Rules};
#[test]
fn test_broadcast_1() {
    let src = "<** @description: Indicate end of playlist. **>
	broadcast endOfPlaylist { }	";
    let result = shared(src, broadcast::<BasicContext>, Rules::broadcast);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_broadcast_2() {
    let src = "broadcast JobRemoved {
		out {
			UInt32 id
			String job
			String result
		}
	}";
    let result = shared(src, broadcast::<BasicContext>, Rules::broadcast);
    assert_eq!(result, (true, src.len() as u32));
}

#[test]
fn test_broadcast_3() {
    let src = "broadcast JobRemoved {
		out {
			UInt32 id
			String job
			String[] result
		}
	}";
    let result = shared(src, broadcast::<BasicContext>, Rules::broadcast);
    assert_eq!(result, (true, src.len() as u32));
}

