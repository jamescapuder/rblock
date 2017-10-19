mod blockchain;

fn main() {
    let mut ch = generate_chain();
    println!("{}", ch);
    println!("{}", ch.validate_chain());
}

fn generate_chain()->blockchain::Chain{
    let mut ch = blockchain::Chain::new();
    for i in (1..100){
        ch.insert_block(i.to_string());
    }
    ch
}
