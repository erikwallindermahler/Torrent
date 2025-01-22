use serde_json::json;
use Torrent::bencodings::decode; // Import your function from the main code

#[test]
fn test_decode() {
    let input = "d3:bar4:spam3:fooi42e4:messli1e1:cee";
    let expected = json!({
        "bar": "spam",
        "foo": 42,
        "mess": [1, "c"]
    });

    let result = decode(input.to_string());

    // Check if the result matches the expected JSON object
    assert_eq!(result, expected);
}