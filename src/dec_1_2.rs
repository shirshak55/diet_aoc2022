fn main() {
    std::fs::read_to_string("./inputs/dec_1.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|w| (!w.is_empty()).then(|| w.parse::<isize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .unwrap()
        .into_iter()
        .fold(vec![vec![]], |mut accum, value| {
            std::iter::once(
                value
                    .is_none()
                    .then(|| accum.push(vec![]))
                    .unwrap_or_else(|| accum.last_mut().unwrap().push(value.unwrap())),
            )
            .last()
            .map(|_| accum)
            .unwrap()
        })
        .into_iter()
        .map(|w: Vec<isize>| w.into_iter().sum())
        .map(|v: isize| Some(v))
        .collect::<Option<Vec<isize>>>()
        .map(|mut v| {
            std::iter::once(v.sort_by(|a, b| b.cmp(a)))
                .last()
                .map(|_| v)
                .unwrap()
        })
        .map(|v| (v[0], v[1], v[2]))
        .map(|v| println!("The partb answer is {}", v.0 + v.1 + v.2))
        .unwrap();
}
