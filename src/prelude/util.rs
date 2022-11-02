pub fn is_valid_tum_id(tum_id: &str) -> bool {
    if tum_id.len() != 7 {
        return false;
    }

    for (i, c) in tum_id.chars().enumerate() {
        if i > 1 && i < 4 {
            if !c.is_numeric() {
                return false;
            }
        } else if !c.is_alphabetic() {
            return false;
        }
    }

    true
}
