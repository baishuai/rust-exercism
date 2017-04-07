type Domino = (usize, usize);


pub fn chain(dominos: &Vec<Domino>) -> Option<Vec<Domino>> {
    if dominos.is_empty() {
        return Some(Vec::new());
    }
    chain_internal(&mut Vec::new(),
                   &mut dominos.iter().map(|&d| Some(d)).collect(),
                   dominos.len())
}


pub fn chain_internal(chain: &mut Vec<Domino>,
                      un_used: &mut Vec<Option<Domino>>,
                      len: usize)
                      -> Option<Vec<Domino>> {
    if chain.len() < len {
        for index in 0..un_used.len() {
            if let Some(domino_orig) = un_used[index] {
                let mut domino = domino_orig;

                if let Some(prev_domino) = chain.last() {
                    if prev_domino.1 == domino.0 {

                    } else if prev_domino.1 == domino.1 {
                        domino = (domino.1, domino.0);
                    } else {
                        continue;
                    }
                }

                un_used[index] = None;
                chain.push(domino);

                let solution = chain_internal(chain, un_used, len);
                if solution.is_some() {
                    return solution;
                }

                chain.pop();
                un_used[index] = Some(domino_orig);
            }
        }
    } else if chain.first().unwrap().0 == chain.last().unwrap().1 {
        return Some(chain.clone());
    }
    None
}