fn main() {
    let s = "ಠ_ಠ";
    assert_eq!(s.len(), 7);
    assert_eq!(s.chars().count(), 3);

    {
        // Unicode

        // Rustの文字と文字列型はUnicodeを対象に設計されている

        {
            // ASCII, Latin-1, Unicode

            fn _latin1_to_char(latin1: u8) -> char {
                latin1 as char
            }

            fn _char_to_latin1(c: char) -> Option<u8> {
                if c as u32 <= 0xff {
                    Some(c as u8)
                } else {
                    None
                }
            }
        }
    }
}
