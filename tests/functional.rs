use base64::{Engine};
use base64::engine::general_purpose;
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
use bonus_prize::utils::constants::{LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID};
use bonus_prize::utils::pdas::{get_draw_result};


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

    let mut draw_data = match general_purpose::STANDARD.decode("Oba9EGPoO/P8/WmrHTJTmB3B/o/+p21xAvRXPWmAtWIly4aP+3IG0QQAAAAAAAAAwZSiZgAAAAAArCP8BgAAAAEAAAAAAAAAEhRQsToCAABxGzAjQwEAAHABjGUGAAAA")
    {
        Ok(data) => data,
        Err(_) => panic!("Error decoding draw data"),
    };
    let draw_account_addres = get_draw_result(4, LOTTERY_ACCOUNT);
    println!("draw_account_addres: {:?}", draw_account_addres);


    // Make the payer the winner
    let payer_bytes = keypair.pubkey().to_bytes();
    for i in 0..32 {
        draw_data[i + 8] = payer_bytes[i];
    }

    program_test.add_account(
        draw_account_addres,
        Account {
            lamports: 1_000_000_000,
            data: draw_data,
            owner: NO_LOSS_LOTTERY_ID,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;



    let ix = create_claim_instruction(
        payer.pubkey(),
        Pubkey::default(),
        LOTTERY_ACCOUNT,
       4
    );
    let mut transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;

    println!("result: {:?}", result.unwrap());
}

