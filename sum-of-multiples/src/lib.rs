use std::collections::BTreeSet;

pub fn sum_of_multiples(num: u32, factors: &[u32]) -> u32 {

    factors.iter()
        .flat_map(|f| (1..((num - 1) / f) + 1).map(move |v| v * f))
        .collect::<BTreeSet<_>>()
        .iter()
        .sum()
    // let mut storage: BTreeSet<u32> = BTreeSet::new();

    // for &f in factors {
    //     let mut factor = f;
    //     while factor < num {
    //         storage.insert(factor);
    //         factor += f;
    //     }
    // }
    // storage.iter().sum()
}