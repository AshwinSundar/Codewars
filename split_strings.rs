// Codewars - Split Strings -> https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/rust
fn split_strings(s: &str) -> Vec<String> {
    let mut split: Vec<String> = Vec::new();
    let mut chars = s.chars();

    for _i in 0..(s.len() / 2) {
        let mut t: String = chars.next().unwrap().to_string();
        t.push(chars.next().unwrap());
        split.push(t);
    }

    // bitwise implementation of modulo 2
    if s.len() & 1 == 1 {
        let mut t: String = chars.next().unwrap().to_string();
        t.push('_');
        split.push(t);
        return split;
    }
    split
}
  
  #[test]
fn test_split_strings() {
    assert_eq!(split_strings("a"), vec!["a_"]);
    assert_eq!(split_strings("ab"), vec!["ab"]);
    assert_eq!(split_strings("abc"), vec!["ab", "c_"]);
    assert_eq!(split_strings("abcd"), vec!["ab", "cd"]);
    assert_eq!(split_strings("abcde"), vec!["ab", "cd", "e_"]);
    assert_eq!(split_strings("abcdef"), vec!["ab", "cd", "ef"]);
}


fn split_strings2(s: &str) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| {
            if c.len() == 1 {
                format!("{}_", c[0])
            } else {
                c.into_iter().collect()
            }
        })
        .collect()
}

#[test]
fn test_split_strings2() {
    assert_eq!(split_strings2("a"), vec!["a_"]);
    assert_eq!(split_strings2("ab"), vec!["ab"]);
    assert_eq!(split_strings2("abc"), vec!["ab", "c_"]);
    assert_eq!(split_strings2("abcd"), vec!["ab", "cd"]);
    assert_eq!(split_strings2("abcde"), vec!["ab", "cd", "e_"]);
    assert_eq!(split_strings2("abcdef"), vec!["ab", "cd", "ef"]);
}
