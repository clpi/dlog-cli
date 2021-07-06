use dlog::Dlog;

fn main() {
    let dlog = Dlog::get();
    println!("{:#?}", dlog);
}
