#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    nucl: Vec<char>
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    nucl: Vec<char>
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let nucleotides = dna
            .chars()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, c)| match c {
                'A' | 'C' | 'G' | 'T' => {
                    acc.push(c);
                    Ok(acc)
                }
                _ => Err(i)
            })?;
            Ok(Dna { nucl: nucleotides })
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.nucl.iter().map(|&c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
             _  =>  unreachable!(),
        }).collect();
        Rna { nucl: rna }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let nucleotides = rna
            .chars()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, c)| match c {
                'A' | 'C' | 'G' | 'U' => {
                    acc.push(c);
                    Ok(acc)
                }
                _ => Err(i)
            })?;
        Ok(Rna { nucl: nucleotides })    
    }
}




