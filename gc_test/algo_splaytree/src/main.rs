use algo_splaytree::{splaytree_insert, splaytree_splay};
use gc_lib::drc::NullableDrc;

fn main() {
    let mut t = splaytree_insert(NullableDrc::null(), 1, 10);
    t = splaytree_insert(t, 2, 20);
    t = splaytree_insert(t, 4, 40);

    for i in 0 ..= 5 {
        t = splaytree_splay(t, i);
        eprintln!("{}: {} -> {}", i, t.key.get(), t.data.get());
    }
}
