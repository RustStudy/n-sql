extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .always_use_colors()
        .set_in_dir("./grammars")
        .set_out_dir("./src")
        .process()
        .unwrap();
}