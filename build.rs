fn main() {
    lalrpop::process_root().expect("Failed to build grammar");

    println!("cargo::rerun-if-changed=src/plushie.lalrpop");
}
