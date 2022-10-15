pub fn lowest_price(books: &[u32]) -> u32 {
    let mut quantities: Vec<u32> = vec![0; 5];

    for book in books {
        quantities[(book - 1) as usize] += 1
    }

    quantities.sort();

    let mut counts: Vec<u32> = vec![0; 5];

    while quantities[4] > 0 {
        let mut count = 0;

        for i in 0..5 {
            if quantities[i] > 0 {
                (count, quantities[i]) = (count + 1, quantities[i] - 1);
            }
        }

        counts[count - 1] += 1;
    }

    while counts[2] > 0 && counts[4] > 0 {
        (counts[2], counts[3], counts[4]) = (counts[2] - 1, counts[3] + 2, counts[4] - 1)
    }

    800 * counts[0] + 1520 * counts[1] + 2160 * counts[2] + 2560 * counts[3] + 3000 * counts[4]
}
