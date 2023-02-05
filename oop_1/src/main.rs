use oop_1::AveragedCollection;

fn main() {
    let mut avg = AveragedCollection::new();
    avg.add(6);
    avg.add(9);

    println!("{}", avg.average())
}
