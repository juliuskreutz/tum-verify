pub fn is_valid_tum_id(tum_id: &str) -> bool {
    let chars: Vec<_> = tum_id.chars().collect();

    if chars.len() != 7 {
        return false;
    }

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let consonants = vec![
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];

    consonants.contains(&chars[0])
        && vowels.contains(&chars[1])
        && chars[2].is_ascii_digit()
        && chars[3].is_ascii_digit()
        && consonants.contains(&chars[4])
        && vowels.contains(&chars[5])
        && consonants.contains(&chars[6])
}
