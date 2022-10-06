pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as i8;
	
    (-n..=n)
        .map(|row| {
            (-n..=n)
                .map(|col| {
                    if row.abs() + col.abs() == n {
                        (col.abs() as u8 + b'A') as char
                    } else {
                        ' '
                    }
					
                })
                .collect()
        })
        .collect()
}