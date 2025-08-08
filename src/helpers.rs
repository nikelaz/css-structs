pub fn is_non_ascii(c: char) -> bool {
  return c as u32 > 127
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ascii_characters() {
    // Test basic ASCII letters
    assert_eq!(is_non_ascii('a'), false);
    assert_eq!(is_non_ascii('Z'), false);
    assert_eq!(is_non_ascii('0'), false);
    assert_eq!(is_non_ascii('9'), false);

    // Test ASCII symbols and punctuation
    assert_eq!(is_non_ascii(' '), false);
    assert_eq!(is_non_ascii('!'), false);
    assert_eq!(is_non_ascii('@'), false);
    assert_eq!(is_non_ascii('~'), false);

    // Test ASCII control characters
    assert_eq!(is_non_ascii('\0'), false); // NULL (0)
    assert_eq!(is_non_ascii('\n'), false); // LF (10)
    assert_eq!(is_non_ascii('\r'), false); // CR (13)
    assert_eq!(is_non_ascii('\t'), false); // TAB (9)
  }

  #[test]
  fn test_boundary_characters() {
    // Test the boundary at 127 (DEL character)
    assert_eq!(is_non_ascii('\x7F'), false); // 127 - still ASCII

    // Test first non-ASCII character
    assert_eq!(is_non_ascii('\u{0080}'), true); // 128 - first non-ASCII
  }

  #[test]
  fn test_non_ascii_characters() {
    // Test extended ASCII / Latin-1
    assert_eq!(is_non_ascii('à'), true); // 224
    assert_eq!(is_non_ascii('ñ'), true); // 241
    assert_eq!(is_non_ascii('ü'), true); // 252

    // Test Unicode characters
    assert_eq!(is_non_ascii('π'), true); // Greek pi
    assert_eq!(is_non_ascii('中'), true); // Chinese character
    assert_eq!(is_non_ascii('🦀'), true); // Crab emoji
    assert_eq!(is_non_ascii('א'), true); // Hebrew character
    assert_eq!(is_non_ascii('🌟'), true); // Star emoji

    // Test some specific Unicode code points
    assert_eq!(is_non_ascii('€'), true); // Euro symbol (8364)
    assert_eq!(is_non_ascii('©'), true); // Copyright symbol (169)
  }

  #[test]
  fn test_edge_cases() {
    // Test characters just above ASCII range
    assert_eq!(is_non_ascii('\u{0081}'), true); // 129
    assert_eq!(is_non_ascii('\u{00A0}'), true); // 160 - non-breaking space
    assert_eq!(is_non_ascii('\u{00FF}'), true); // 255 - ÿ

    // Test some higher Unicode ranges
    assert_eq!(is_non_ascii('\u{1000}'), true); // Myanmar script
    assert_eq!(is_non_ascii('\u{10000}'), true); // High Unicode plane
  }
}
