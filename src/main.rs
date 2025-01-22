use Torrent::bencodings;

fn main() {
    let test = bencodings::decode("d3:bar4:spam3:fooi42e4:messli1e1:cee".to_string());
    println!("{:?}", test);
}