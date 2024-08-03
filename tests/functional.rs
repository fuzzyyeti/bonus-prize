use bonus_prize::instruction::create_claim_instruction;
use bonus_prize::utils::constants::LOTTERY_ACCOUNT;
use solana_program::program_pack::Pack;
use spl_associated_token_account::get_associated_token_address;
use {
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

mod setup;

#[tokio::test]
async fn test_lamport_transfer() {
    let (mut banks_client, payer, recent_blockhash, prize_mint, bonus_prize_seed_singer) =
        setup::setup().await;
    // Claim bonus prize

    let ix = create_claim_instruction(payer.pubkey(), prize_mint, LOTTERY_ACCOUNT, 4);
    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    // Assert that tokens are in vault
    let vault_ata = get_associated_token_address(&bonus_prize_seed_singer, &prize_mint);
    let vault_account = banks_client.get_account(vault_ata).await.unwrap().unwrap();
    let vault_account_data = spl_token::state::Account::unpack(&vault_account.data).unwrap();
    assert_eq!(vault_account_data.amount, 1_000_000_000);

    let result = banks_client.process_transaction(transaction).await;
    println!("result: {:?}", result.unwrap());

    // Assert that tokens are now in the claimer
    let claimer_ata = get_associated_token_address(&payer.pubkey(), &prize_mint);
    let claimer_account = banks_client
        .get_account(claimer_ata)
        .await
        .unwrap()
        .unwrap();
    let claimer_account_data = spl_token::state::Account::unpack(&claimer_account.data).unwrap();
    assert_eq!(claimer_account_data.amount, 1_000_000_000);
}
