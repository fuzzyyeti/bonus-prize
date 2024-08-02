use base64::Engine;
use base64::engine::general_purpose;
use solana_program::instruction::Instruction;
use solana_sdk::signature::Keypair;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::transfer;
use {
    bonus_prize::processor::process_instruction,
    solana_program::pubkey::Pubkey,
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    std::str::FromStr,
};
use bonus_prize::instruction::create_claim_instruction;
use bonus_prize::utils::constants::{LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID};
use bonus_prize::utils::pdas::{get_bonus_prize_seed_signer, get_draw_result};
use crate::setup::create_token;

mod setup;


#[tokio::test]
async fn test_lamport_transfer() {
    let program_id = Pubkey::from_str("54oykPNNXxpXihbuU5H6j3MZmqCxaAdHALDvVYfzwnW4").unwrap();
    let source_pubkey = Pubkey::new_unique();
    let prize_mint = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "bonus_prize",
        program_id,
        processor!(process_instruction),
    );
    let keypair = solana_sdk::signature::Keypair::new();
    program_test.add_account(
        keypair.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );
    program_test.add_account(
        source_pubkey,
        Account {
            lamports: 5_000,
            owner: program_id, // Can only withdraw lamports from accounts owned by the program
            ..Account::default()
        },
    );
    program_test.add_account(
        prize_mint,
        Account {
            lamports: 890_875_000,
            ..Account::default()
        },
    );

    let prize_adder = Keypair::new();
    program_test.add_account(
        prize_adder.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );

    let mut draw_data = match general_purpose::STANDARD.decode("Oba9EGPoO/P8/WmrHTJTmB3B/o/+p21xAvRXPWmAtWIly4aP+3IG0QQAAAAAAAAAwZSiZgAAAAAArCP8BgAAAAEAAAAAAAAAEhRQsToCAABxGzAjQwEAAHABjGUGAAAA")
    {
        Ok(data) => data,
        Err(_) => panic!("Error decoding draw data"),
    };
    let draw_account_address = get_draw_result(4, LOTTERY_ACCOUNT);

    // Make the payer the winner
    let payer_bytes = keypair.pubkey().to_bytes();
    for i in 0..32 {
        draw_data[i + 8] = payer_bytes[i];
    }

    program_test.add_account(
        draw_account_address,
        Account {
            lamports: 1_000_000_000,
            data: draw_data,
            owner: NO_LOSS_LOTTERY_ID,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let (prize_mint, prize_adder_ada) = create_token(&prize_adder, &mut banks_client, &payer, recent_blockhash).await;

    // Add the prize
    let bonus_prize_seed_singer = get_bonus_prize_seed_signer(4, LOTTERY_ACCOUNT);
    let mut add_prize_ixs : Vec<Instruction> = Vec::new();
    add_prize_ixs.push(create_associated_token_account(
        &prize_adder.pubkey(),
        &bonus_prize_seed_singer,
        &prize_mint,
        &spl_token::id()
    ));
    let bonus_prize_ata = get_associated_token_address(&bonus_prize_seed_singer, &prize_mint);

    add_prize_ixs.push(transfer(
        &spl_token::id(),
        &prize_adder_ada,
        &bonus_prize_ata,
        &prize_adder.pubkey(),
        &[],
        1_000_000_000
    ).unwrap());

    let transaction = Transaction::new_signed_with_payer(
        &add_prize_ixs,
        Some(&payer.pubkey()),
        &[&prize_adder],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;

    // Claim bonus prize

    let ix = create_claim_instruction(
        payer.pubkey(),
        prize_mint,
        LOTTERY_ACCOUNT,
       4
    );
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;

    println!("result: {:?}", result.unwrap());
}

