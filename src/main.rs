use crate::libs::job::DorkGen;
mod libs;

fn main() {
    let mut dork: DorkGen = DorkGen::new("keyword_2.txt".to_string());
    dork.run_gen();
}
