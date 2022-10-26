use std::iter;

const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];

    let mut movement = VECTORS.iter().cycle();

    let (mut x, mut y, mut n) = (-1, 0, 1..);

    for (move_x, move_y) in iter::once(size)
        .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
        .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
    {
        x += move_x;

        y += move_y;

        matrix[y as usize][x as usize] = n.next().unwrap();
    }

    matrix
}
