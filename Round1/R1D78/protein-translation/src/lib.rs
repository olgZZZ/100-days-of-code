const STOP: &str = "stop codon";
pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.iter().find(|&n| n.0 == codon).map(|n| n.1)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chars| chars.iter().collect::<String>())
        .map(|codon| self.name_for(&codon))
        .take_while(|codon| codon != &Some(STOP))
        .collect()      
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs: pairs.into_iter().collect() }
}

