// Codewars Kata 3 - https://www.codewars.com/kata/588e68aed4cff457d300002e/train/rust
fn turn(current: char, target: char) -> String {
    match (current, target) {
        ('N', 'E') | ('E', 'S') | ('S', 'W') | ('W', 'N') => "right",
        _ => "left",
    }
    .to_string()
}

#[test]
fn test_turn() {
    assert_eq!(turn('N', 'E'), "right");
    assert_eq!(turn('N', 'W'), "left");
    assert_eq!(turn('E', 'S'), "right");
    assert_eq!(turn('E', 'N'), "left");
    assert_eq!(turn('S', 'W'), "right");
    assert_eq!(turn('S', 'E'), "left");
    assert_eq!(turn('W', 'N'), "right");
    assert_eq!(turn('W', 'S'), "left");
}
