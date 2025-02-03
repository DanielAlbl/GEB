use crate::miu::Miu;

mod miu;

fn main() {
    let mut miu = Miu::new(&["MI".to_string()]);
    miu.derive(3);
    miu.show_theorems();
}
