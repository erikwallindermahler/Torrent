use Torrent::bencodings;
use Torrent::libtorrent;

fn main() {
    let test = bencodings::decode("li1eli2eli3eli4eli5ed4:testli2eeeeeeee".as_bytes().to_vec());
    println!("{:?}", test);
    //libtorrent::load_torrent_file("torrenttest.torrent".to_string());
}