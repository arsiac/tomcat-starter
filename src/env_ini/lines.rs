/// 是否是空白行
pub fn is_blank(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    for c in s.chars() {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

/// 是否是注释行
pub fn is_comment(s: &str) -> bool {
    if s.is_empty() {
        false
    } else {
        let bytes = s.as_bytes();
        let b = bytes[0];
        b == b';' || b == b'#'
    }
}

/// 是否是 section
pub fn is_section(tl: &str) -> bool {
    let bytes = tl.as_bytes();
    bytes[0] == b'[' && bytes[bytes.len() - 1] == b']'
}

/// 值是否结束
pub fn is_value_end(tl: &str) -> bool {
    let bytes = tl.as_bytes();
    bytes[bytes.len() - 1] != b'\\'
}

/// 是否空白开头
pub fn start_with_blank(line: &str) -> bool {
    match line.chars().next() {
        None => false,
        Some(c) => c.is_whitespace(),
    }
}