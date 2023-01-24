use pathify::base;

fn main() {
    let mut pathify = base("./src/lib.rs");
    let mut file2 = pathify.add_node("./main.rs");
    println!("{:#?}", pathify.get());
    println!("{:#?}", file2.get());
}
