fn main() {
    std::fs::read_to_string("./inputs/dec_3.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|v| v.split_at(v.len() / 2))
                .map(|w| {
                    (
                        std::collections::HashSet::from_iter(w.0.chars()),
                        std::collections::HashSet::from_iter(w.1.chars()),
                    )
                })
                .collect::<Vec<_>>()
        })
        .map(
            |v: Vec<(
                std::collections::HashSet<char>,
                std::collections::HashSet<char>,
            )>| {
                v.into_iter()
                    .map(|w| (w.0.intersection(&w.1).map(|v| *v).collect::<Vec<_>>()))
                    .collect::<Vec<_>>()
            },
        )
        .map(|w| {
            w.into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|y| {
                            vec![
                                y as isize - 'a' as isize + 1,
                                y as isize - 'A' as isize + 27,
                            ]
                        })
                        .flatten()
                        .filter(|&v| v >= 1 && v <= 52)
                        .sum::<isize>()
                })
                .sum::<isize>()
        })
        .map(|v| println!("The answer is {v}"))
        .unwrap()
}
