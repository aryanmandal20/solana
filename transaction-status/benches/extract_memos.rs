#![feature(test)]

extern crate test;

use {
    solana_sdk::{instruction::CompiledInstruction, message::Message, pubkey::Pubkey},
    solana_transaction_status::extract_memos::{spl_memo_id_v1, spl_memo_id_v3, ExtractMemos},
    test::Bencher,
};

#[bench]
fn bench_extract_memos(b: &mut Bencher) {
    // Precompute constants
    const MEMO_V1: Pubkey = spl_memo_id_v1();
    const MEMO_V3: Pubkey = spl_memo_id_v3();
    const MEMO: &str = "Test memo";
    const MEMO_BYTES: &[u8] = MEMO.as_bytes();

    // Pre-allocate memory for account_keys
    let mut account_keys = Vec::with_capacity(64);
    for _ in 0..64 {
        account_keys.push(Pubkey::new_unique());
    }
    account_keys[62] = MEMO_V1;
    account_keys[63] = MEMO_V3;

    // Pre-allocate memory for instructions
    let mut instructions = Vec::with_capacity(20);
    for i in 0..20 {
        let program_id_index = 62 + (i % 2);
        instructions.push(CompiledInstruction {
            program_id_index,
            accounts: vec![],
            data: MEMO_BYTES.to_vec(),
        });
    }

    let message = Message {
        account_keys,
        instructions,
        ..Message::default()
    };

    // Benchmark the extraction of memos
    b.iter(|| message.extract_memos());
}
