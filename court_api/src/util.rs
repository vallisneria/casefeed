pub fn remove_bracket(before: &String) -> String {
    before
        .replace("[", "")
        .replace("]", "")
        .replace("〈", "")
        .replace("〉", "")
}

pub fn replace_middle_dot(before: &String) -> String {
    before.replace("ㆍ", "·")
}
