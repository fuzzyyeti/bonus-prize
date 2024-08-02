use std::cmp::min;
use solana_sdk::signature::{Keypair, SeedDerivable, Signer};
use solana_program_test::BanksClient;
use solana_program::hash::Hash;
use solana_program::instruction::Instruction;
use spl_token::state::Mint;
use spl_token::instruction::{initialize_mint2, mint_to};
use solana_sdk::transaction::Transaction;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;

pub async fn create_token(prize_adder: &Keypair, banks_client: &mut BanksClient, payer: &Keypair, recent_blockhash: Hash) -> (Pubkey, Pubkey) {
    let mint_key = Keypair::from_seed(&[1u8; 32]).unwrap();
    let prize_adder_ata = spl_associated_token_account::get_associated_token_address(&prize_adder.pubkey(), &mint_key.pubkey());

    let mut ixs: Vec<Instruction> = Vec::new();
    let min_rent = banks_client.get_rent().await.unwrap().minimum_balance(Mint::LEN);
    println!("mint_key: {:?}", mint_key.pubkey());
    println!("payer: {:?}", payer.pubkey());
    ixs.push(solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint_key.pubkey(),
        min_rent,
        Mint::LEN as u64,
        &spl_token::id(),
    ));
    ixs.push(initialize_mint2(
        &spl_token::id(),
        &mint_key.pubkey(),
        &payer.pubkey(),
        None,
        9
    ).unwrap());
    ixs.push(spl_associated_token_account::instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_key.pubkey(),
        &spl_token::id()
    ));
    ixs.push(mint_to(
        &spl_token::id(),
        &mint_key.pubkey(),
        &prize_adder_ata,
        &prize_adder.pubkey(),
        &[&payer.pubkey()],
        1_000_000_000
    ).unwrap());

    let transaction = Transaction::new_signed_with_payer(
        &ixs,
        Some(&payer.pubkey()),
        &[&payer, &mint_key],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;
    println!("result: {:?}", result);
    (mint_key.pubkey(), prize_adder_ata)
}
