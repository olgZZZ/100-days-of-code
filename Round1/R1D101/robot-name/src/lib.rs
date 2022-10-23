use rand;
use rand::Rng;
pub struct Robot {
    name: String,
}

static mut USED: Vec<String> = vec![];
   
fn generate_name() -> String {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    (0..2)
        .map(|_| rng1.gen_range(0..26))
        .map(|c| c + 'A' as u8)
        .chain((0..3).map(|_| rng2.gen_range(0..10) + '0' as u8))
        .map(|c| c as char)
        .collect()
}

impl Robot {
    pub fn new() -> Self {
        let mut name = generate_name();
        unsafe {
            while USED.contains(&name) {
                name = generate_name();
            }
            USED.push(name.clone());
        }
        Robot { name }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = generate_name();
    }
}