fn main() {
    std::fs::read_to_string("./inputs/dec_4.txt")
        .ok()
        .map(|v| {
            v.lines()
                .map(|w| w.split(|c| c == ',' || c == '-').collect::<Vec<_>>())
                .map(|x| {
                    (
                        x[0].parse::<isize>().unwrap(),
                        x[1].parse::<isize>().unwrap(),
                        x[2].parse::<isize>().unwrap(),
                        x[3].parse::<isize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .map(|w| {
            w.into_iter()
                .map(|z| (z.0 <= z.2 && z.1 >= z.3) || (z.2 <= z.0 && z.3 >= z.1))
                .collect::<Vec<_>>()
        })
        .map(|w| println!("The answer is {}", w.iter().filter(|v| **v).count()))
        .unwrap()
}
