use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::bencodings;
use crate::bencodings::BencodeValue;

/// Contains torrent data types.
enum TorrentContent {
    String(String),
    Integer(i64),
    Vec(u8),
}

/// Holder of .torrent data. May be extended or made into a class
struct Torrent {
    announce: String,
    info: HashMap<String, TorrentContent>,
}

/// Loads a torrent file from given PATH
/// Returns a torrent object, containing the extracted information
pub fn load_torrent_file(path : String) {
    let mut torrent_file = File::open(path).unwrap();
    let mut contents = Vec::new();
    torrent_file.read_to_end(&mut contents).unwrap();
    let torrent_map: BencodeValue = bencodings::decode(contents.to_vec());
    println!("{:?}", &torrent_map);
}