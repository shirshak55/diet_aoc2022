fn main() {
    std::fs::read_to_string("./inputs/dec_3.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|w| std::collections::HashSet::from_iter(w.chars()))
                .collect::<Vec<std::collections::HashSet<char>>>()
        })
        .map(|w| {
            w.chunks(3)
                .into_iter()
                .map(|v| Vec::from(v.clone()))
                // .map(|s| dbg!(s))
                .collect::<Vec<Vec<_>>>()
        })
        .map(|v: Vec<Vec<std::collections::HashSet<char>>>| {
            v.into_iter()
                .map(|w| {
                    (w[0]
                        .intersection(&std::collections::HashSet::from_iter(
                            w[1].intersection(&w[2]).map(|v| *v),
                        ))
                        .map(|v| *v)
                        .collect::<Vec<_>>())
                })
                .collect::<Vec<_>>()
        })
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
