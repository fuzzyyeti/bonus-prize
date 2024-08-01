use base64::{decode, Engine};
use base64::engine::general_purpose;
use solana_program::msg;
use {
    solana_program::{
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    bonus_prize::processor::process_instruction,
    std::str::FromStr,
};
use bonus_prize::instruction::create_claim_instruction;
use bonus_prize::utils::pdas::{LOTTERY_ACCOUNT};


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

    let draw_data = match general_purpose::STANDARD.decode("Oba9EGPoO/P8/WmrHTJTmB3B/o/+p21xAvRXPWmAtWIly4aP+3IG0QQAAAAAAAAAwZSiZgAAAAAArCP8BgAAAAEAAAAAAAAAEhRQsToCAABxGzAjQwEAAHABjGUGAAAA")
    {
        Ok(data) => data,
        Err(_) => panic!("Error decoding draw data"),
    };

    let data = &draw_data[8..];
    // Extract the winner Pubkey (32 bytes)
    let winner_bytes: [u8; 32] = data[..32].try_into().unwrap();
    let winner = Pubkey::new_from_array(winner_bytes);
    // Extract the draw (u64, 8 bytes)
    let draw_bytes: [u8; 8] = data[32..40].try_into().unwrap();
    let draw = u64::from_le_bytes(draw_bytes);
    msg!("winner: {:?} draw: {:?}", winner, draw);
    // Skip the first 8 bytes (discriminator)

    let data = &draw_data.as_slice()[8..];




    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;



    let ix = create_claim_instruction(
        payer.pubkey(),
        Pubkey::default(),
        LOTTERY_ACCOUNT,
        4
    );
    let  transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;

    println!("result: {:?}", result.unwrap());
}

