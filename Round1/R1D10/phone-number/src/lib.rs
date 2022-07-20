pub fn number(user_number: &str) -> Option<String> {
    let mut phone: Vec<char> = user_number.chars()
        .filter(|c| c.is_digit(10))
        .collect();

    if phone[0] == '1' { 
        phone.remove(0); 
    } 

    if phone.len() != 10 || phone[0] < '2' || phone[3] < '2' { 
        return None; 
    }

    Some (phone.into_iter().collect::<String>())

}