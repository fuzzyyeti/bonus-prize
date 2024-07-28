use base64::{decode, Engine};
use base64::engine::general_purpose;
use borsh::BorshDeserialize;
use solana_program::msg;
use {
    solana_program::{
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{account::Account, signature::Signer, transaction::Transaction},
    spl_example_transfer_lamports::processor::process_instruction,
    std::str::FromStr,
};
use spl_example_transfer_lamports::instructions::create_add_prizes_instruction;
use spl_example_transfer_lamports::state::PRIZE_MINTS_SEED;
use spl_example_transfer_lamports::utils::pdas::{get_draw_results_pda, LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID};


#[tokio::test]
async fn test_lamport_transfer() {
    let program_id = Pubkey::from_str("TransferLamports111111111111111111111111111").unwrap();
    let source_pubkey = Pubkey::new_unique();
    let prize_mint = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "spl_example_transfer_lamports",
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


    let draw_results = get_draw_results_pda(4, LOTTERY_ACCOUNT);
    program_test.add_account(
        draw_results,
        Account {
            lamports: 1_000_000_000,
            owner: NO_LOSS_LOTTERY_ID,
            data: draw_data,
            ..Account::default()
        },
    );


    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;



    let ix = create_add_prizes_instruction(
        &program_id,
        1,
        draw_results,
        vec![prize_mint, prize_mint],
        payer.pubkey(),
    );
    let  transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let result = banks_client.process_transaction(transaction).await;
    let (prize_mints_account, _) = Pubkey::find_program_address(
        &[
            &PRIZE_MINTS_SEED,
            &draw_results.to_bytes(),
            &1u64.to_le_bytes(),
        ],
        &program_id,
    );
    println!("prize_mints_account: {:?}", prize_mints_account);
    let prize_mints_account = match banks_client.get_account(prize_mints_account).await {
        Ok(Some(account)) => account,
        _ => panic!("prize_mints_account not found"),
    };
    println!("prize_mints_account: {:?}", prize_mints_account.data.len());
    let prize_mints_data = spl_example_transfer_lamports::state::PrizeMints::try_from_slice(&prize_mints_account.data).unwrap();
    println!("prize_mints_data: {:?}", prize_mints_data);

    println!("result: {:?}", result.unwrap());
}

