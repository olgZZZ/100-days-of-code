pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field = minefield.to_vec();
    let mut res: Vec<String> = Vec::new();

    for y in 0..field.len() {
        let mut row = String::new();

        for x in 0..field[y].len() {
            row.push(annotate_cell(x, y, &field));
        }

        res.push(row);
    }

    res
}

fn annotate_cell(x: usize, y: usize, field: &Vec<&str>) -> char {
    let curr_cell = field[y].chars().nth(x).unwrap();

    if curr_cell == '*' { 
        return curr_cell;
    }

    let count = count(x, y, field);

    if count > 0 {
        count.to_string().chars().nth(0).unwrap()
    } else {
        ' '
    }
}

fn count(x_size: usize, y_size: usize, field: &Vec<&str>) -> usize {
    let mut count = 0;
    let x = x_size as i8;
    let y = y_size as i8;

    for xs in x - 1..x + 2 {
        for ys in y - 1..y + 2 {
            count += mines_at(xs, ys, field);
        }
    }

    count
}

fn mines_at(x: i8, y: i8, field: &Vec<&str>) -> usize {
    if x < 0 || y < 0 {
        return 0;
    }

    let x_size = x as usize;
    let y_size = y as usize;

    if y_size >= field.len() {
        return 0;
    }

    match field[y_size].chars().nth(x_size) {
        Some(ch) => {
            if ch == '*' {
                1
            } else {
                0
            }
        }

        None => 0,
    }
}
