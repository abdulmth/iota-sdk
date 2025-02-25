// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::{
    types::block::output::{
        feature::SenderFeature,
        unlock_condition::{AddressUnlockCondition, ExpirationUnlockCondition},
        BasicOutputBuilder, UnlockCondition,
    },
    wallet::Result,
};

use crate::wallet::common::{create_accounts_with_funds, make_wallet, setup, tear_down};

#[ignore]
#[tokio::test]
async fn balance_expiration() -> Result<()> {
    let storage_path = "test-storage/balance_expiration";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account_0 = &create_accounts_with_funds(&wallet, 1).await?[0];
    let account_1 = wallet.create_account().finish().await?;
    let account_2 = wallet.create_account().finish().await?;

    let seconds_until_expired = 20;
    let token_supply = account_0.client().get_token_supply().await?;
    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            // Send to account 1 with expiration to account 2, both have no amount yet
            .with_unlock_conditions(vec![
                UnlockCondition::Address(AddressUnlockCondition::new(
                    *account_1.addresses().await?[0].address().as_ref(),
                )),
                UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                    *account_2.addresses().await?[0].address().as_ref(),
                    // Current time + 20s
                    account_0.client().get_time_checked().await? + seconds_until_expired,
                )?),
            ])
            .with_features(vec![SenderFeature::new(
                *account_0.addresses().await?[0].address().as_ref(),
            )])
            .finish_output(token_supply)?,
    ];

    let balance_before_tx = account_0.balance().await?;
    let tx = account_0.send(outputs, None).await?;
    let balance_after_tx = account_0.balance().await?;
    // Total doesn't change before syncing after tx got confirmed
    assert_eq!(
        balance_before_tx.base_coin().total(),
        balance_after_tx.base_coin().total()
    );
    assert_eq!(balance_after_tx.base_coin().available(), 0);

    account_0
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;

    // Account 1 balance before expiration
    let balance = account_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Account 2 balance before expiration
    let balance = account_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 1);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Wait until expired
    tokio::time::sleep(std::time::Duration::from_secs(seconds_until_expired.into())).await;

    // Account 1 balance after expiration
    let balance = account_1.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 0);
    assert_eq!(balance.base_coin().available(), 0);

    // Account 2 balance after expiration
    let balance = account_2.sync(None).await?;
    assert_eq!(balance.potentially_locked_outputs().len(), 0);
    assert_eq!(balance.base_coin().total(), 1_000_000);
    assert_eq!(balance.base_coin().available(), 1_000_000);

    // It's possible to send the expired output
    let outputs = vec![
        BasicOutputBuilder::new_with_amount(1_000_000)
            // Send to account 1 with expiration to account 2, both have no amount yet
            .with_unlock_conditions(vec![AddressUnlockCondition::new(
                *account_1.addresses().await?[0].address().as_ref(),
            )])
            .finish_output(token_supply)?,
    ];
    let _tx = account_2.send(outputs, None).await?;

    tear_down(storage_path)
}

#[ignore]
#[tokio::test]
#[cfg(feature = "participation")]
async fn balance_voting_power() -> Result<()> {
    let storage_path = "test-storage/balance_voting_power";
    setup(storage_path)?;

    let wallet = make_wallet(storage_path, None, None).await?;

    let account = &create_accounts_with_funds(&wallet, 1).await?[0];

    let faucet_amount = 100_000_000_000;

    let balance = account.balance().await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount);

    let voting_power = 1_000_000;
    // Only use a part as voting power
    let tx = account.increase_voting_power(voting_power).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), faucet_amount - voting_power);
    let account_voting_power = account.get_voting_power().await?;
    assert_eq!(account_voting_power, voting_power);

    // Increase voting power to total amount
    let tx = account.increase_voting_power(faucet_amount - voting_power).await?;
    account
        .retry_transaction_until_included(&tx.transaction_id, None, None)
        .await?;
    let balance = account.sync(None).await?;
    assert_eq!(balance.base_coin().total(), faucet_amount);
    assert_eq!(balance.base_coin().available(), 0);
    let account_voting_power = account.get_voting_power().await?;
    assert_eq!(account_voting_power, faucet_amount);

    tear_down(storage_path)
}
