

#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid(String);

#[derive(Debug)]
pub struct DeoxyribonucleicAcid(String);

impl RibonucleicAcid {
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid(String::from(s))
    }
}


impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid(String::from(s))
    }

    pub fn to_rna(&self) -> RibonucleicAcid {

        RibonucleicAcid::new(self.0
            .chars()
            .map(|n| {
                match n {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => ' ',
                }
            })
            .collect::<String>()
            .as_str())
    }
}