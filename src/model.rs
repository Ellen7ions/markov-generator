use rand::Rng;

use crate::slicer::Slicer;
use crate::typo::*;
use std::collections::HashMap;
pub struct Model {
    markov: HashMap<Context, HashMap<Character, usize>>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            markov: HashMap::new(),
        }
    }

    pub fn train(&mut self, contents: &Vec<Character>) {
        let slicer = Slicer::new(contents);
        for (context, next_char) in slicer {
            
            self.markov
                .entry(context)
                .and_modify(|prob| {
                    prob.entry(next_char).and_modify(|c| *c += 1).or_insert(1);
                })
                .or_insert({
                    let mut prob = HashMap::new();
                    prob.insert(next_char, 1);
                    prob
                });
        }
    }

    pub fn show_prob(&self) {
        for (k, v) in &self.markov {
            println!("context: {:x} => {}", k, v.len());
            println!("\n");
        }
    }

    pub fn predict(&self, context: &mut Context) -> Option<Character> {
        let mut rng = rand::thread_rng();
        if let Some(v) = self.markov.get(&context) {
            let keys = v.keys().collect::<Vec<&Character>>();
            let vals = v.values().collect::<Vec<&usize>>();
            let total = v.values().sum::<usize>();
            let p = rng.gen_range(0..=total);
            let mut _sum = 0;
            for i in 0..keys.len() {
                _sum += vals[i];
                if _sum >= p {
                    let res = *keys[i];
                    *context = ((*context) << 8) | (res as Context);
                    return Some(res);
                }
            }
        }
        None
    }

    pub fn save_model(&mut self, path: &str) {
        todo!()
    }

    pub fn load_model(&mut self, path: &str) {
        todo!()
    }
}
