use std::cmp::min;
use std::collections::HashSet;

pub struct Miu {
    theorems: HashSet<String>,
    new_theorems: HashSet<String>
}

impl Miu {
    pub fn new(axioms: &[String]) -> Miu {
        Miu {
            theorems: axioms.iter().cloned().collect(),
            new_theorems: axioms.iter().cloned().collect(),
        }
    }

    pub fn derive(&mut self, steps: usize) {
        for _ in 0..steps {
            let new_theorems = self.new_theorems.clone();
            self.new_theorems.clear();
            for theorem in new_theorems {
                self.rule1(&theorem);
                self.rule2(&theorem);
                self.rule3(&theorem);
                self.rule4(&theorem);
            }
            self.theorems.extend(self.new_theorems.clone().into_iter());
        }
    }

    pub fn show_theorems(&self) {
        for theorem in &self.theorems {
            println!("{}", theorem);
        }
    }

    fn rule1(&mut self, theorem: &str) {
        if theorem.ends_with("I") {
            self.new_theorems.insert(format!("{}{}", theorem, "U"));
        }
    }

    fn rule2(&mut self, theorem: &str) {
        if theorem.starts_with("M") {
            let x = &theorem[1..];
            self.new_theorems.insert(format!("M{}{}", x, x));
        }
    }

    fn rule3(&mut self, theorem: &str) {
        for i in 0..theorem.len() {
            let j = min(i + 3, theorem.len());
            if &theorem[i..j] == "III" {
                self.new_theorems.insert(format!("{}{}{}", &theorem[..i], "U", &theorem[j..]));
            }
        }
    }

    fn rule4(&mut self, theorem: &str) {
        for i in 0..theorem.len() {
            let j = min(i + 2, theorem.len());
            if &theorem[i..j] == "UU" {
                self.new_theorems.insert(format!("{}{}", &theorem[..i], &theorem[j..]));
            }
        }
    }
}
