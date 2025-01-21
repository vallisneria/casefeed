pub(crate) fn replace_middle_dot(before: &String) -> String {
    before.replace("ㆍ", "·")
}

pub(crate) fn remove_bracket(before: &String) -> String {
    before
        .replace("[", "")
        .replace("]", "")
        .replace("〈", "")
        .replace("〉", "")
}
