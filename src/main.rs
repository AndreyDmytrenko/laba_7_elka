fn draw_tree(triangles: usize) {
    for i in 1..=triangles {
        for j in 1..=i {
            let stars = 2 * j - 1;
            let spaces = triangles + triangles - stars;
            let line = std::iter::repeat(' ').take(spaces / 2)
                .chain(std::iter::repeat('*').take(stars))
                .collect::<String>();
            println!("{}", line);
        }
    }
}

fn main() {
    let triangles = 6; // Кількість трикутників для ялинки
    draw_tree(triangles);
}