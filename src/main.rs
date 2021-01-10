use blockchainlib::*;

fn main() {
    let difficulty = 0x006fffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Henry".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 20,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block: {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block.");

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            last_hash,
            vec![Transaction {
                inputs: vec![],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Henry".to_owned(),
                        value: 50,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 20,
                    },
                ],
            }],
            difficulty,
        );
        block.mine();
        println!("Mined genesis block: {:?}", &block);
        last_hash = block.hash.clone();
        blockchain
            .update_with_block(block)
            .expect("Failed to add block.");
    }
}
