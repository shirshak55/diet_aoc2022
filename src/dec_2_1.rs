fn main() {
    std::fs::read_to_string("./inputs/dec_2.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|w| w.chars())
                .map(|mut v| {
                    (
                        v.next().unwrap() as usize - 'A' as usize,
                        v.skip(1).next().unwrap() as usize - 'X' as usize,
                    )
                })
                .collect::<Vec<_>>()
        })
        .map(|w| (w, [[4, 8, 3], [1, 5, 9], [7, 2, 6]]))
        .map(|v| {
            v.0.into_iter()
                .map(|w| v.1[(w.0)][(w.1)])
                .collect::<Vec<_>>()
        })
        .map(|v| v.into_iter().sum())
        .map(|v: isize| println!("The answer is {}", v))
        .unwrap();
}
