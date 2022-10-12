const SHARP_NOTES: &[&str] = &[
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLAT_NOTES: &[&str] = &[
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

#[derive(Debug)]
pub struct Error;

pub struct Scale {
    scale: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let chromatic_verient: &[&str] = match tonic {
            "C" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#"
            | "a" => SHARP_NOTES,

            _ => FLAT_NOTES,
        };

        let mut next_note = chromatic_verient
            .iter()
            .position(|&note| note.to_uppercase() == tonic.to_uppercase())
            .unwrap();

        let mut scale = vec![chromatic_verient[next_note].to_string()];

        for interval in intervals.chars() {
            next_note += match interval {
                'm' => 1,

                'M' => 2,

                'A' => 3,

                _ => continue,
            };

            scale.push(chromatic_verient[next_note % 12].to_string())
        }

        Ok(Scale { scale })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    // Use Sharps: G, D, A, E, B, F# major e, b, f#, c#, g#, d# minor

    // Use Flats: F, Bb, Eb, Ab, Db, Gb major d, g, c, f, bb, eb minor

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
