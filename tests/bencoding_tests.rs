use std::collections::HashMap;
use Torrent::bencodings::{decode, BencodeValue};


#[test]
fn test_decode() {
    let input = b"d3:bar4:spam3:fooi42e4:messli1e1:cee";
    let mut map = HashMap::new();
    map.insert("bar".to_string(), BencodeValue::String("spam".to_string()));
    map.insert("foo".to_string(), BencodeValue::Integer(42));
    map.insert("mess".to_string(), BencodeValue::List(vec![BencodeValue::Integer(1), BencodeValue::String("c".to_string())]));

    let expected = BencodeValue::Dictionary(map);
    let result = decode(input.to_vec());
    assert_eq!(result, expected);
}