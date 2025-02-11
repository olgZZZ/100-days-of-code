#[allow(dead_code)]

fn main() {
    // Latin-1 to char
    fn latin1_to_char(latin1: u8) -> char {
        latin1 as char
    }

    // char to Latin-1 (0 - 0x7f)
    fn char_to_latin1(c: char) -> Option<u8> {
        if c as u32 <= 0x7f {
            Some(c as u8)
        } else {
            None
        }
    }

    if 0b000_011111_100110_000000 == 0x1f980 {
        println!("{} (crab emoji)", '\u{1F980}');
    }

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");


    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());

    // Поскольку String::from_utf8 завершился ошибкой, исходный вектор
    // не потреблен, и мы можем получить его из значения ошибки.
    assert_eq!(result.unwrap_err().into_bytes(),
    vec![0x9f, 0xf0, 0xa6, 0x80]);

}
