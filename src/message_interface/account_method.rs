// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::{PreparedTransactionDataDto, SignedTransactionDataDto},
    block::{
        output::{
            dto::{AliasIdDto, NativeTokenDto, NftIdDto, OutputDto, TokenIdDto, TokenSchemeDto},
            feature::dto::FeatureDto,
            unlock_condition::dto::UnlockConditionDto,
            FoundryId, OutputId,
        },
        payload::transaction::TransactionId,
    },
};
use serde::Deserialize;

use crate::{
    account::operations::{
        address_generation::AddressGenerationOptions,
        output_claiming::OutputsToClaim,
        syncing::SyncOptions,
        transaction::{
            high_level::minting::{mint_native_token::NativeTokenOptionsDto, mint_nfts::NftOptionsDto},
            prepare_output::OutputOptionsDto,
            TransactionOptions,
        },
    },
    message_interface::dtos::{AddressWithAmountDto, AddressWithMicroAmountDto},
    AddressAndNftId, AddressNativeTokens,
};

/// Each public account method.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "name", content = "data")]
pub enum AccountMethod {
    /// Build an AliasOutput.
    /// Expected response: [`Output`](crate::message_interface::Response::Output)
    #[allow(missing_docs)]
    BuildAliasOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "aliasId")]
        alias_id: AliasIdDto,
        #[serde(rename = "stateIndex")]
        state_index: Option<u32>,
        #[serde(rename = "stateMetadata")]
        state_metadata: Option<Vec<u8>>,
        #[serde(rename = "foundryCounter")]
        foundry_counter: Option<u32>,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build a BasicOutput.
    /// Expected response: [`Output`](crate::message_interface::Response::Output)
    #[allow(missing_docs)]
    BuildBasicOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
    },
    /// Build a FoundryOutput.
    /// Expected response: [`Output`](crate::message_interface::Response::Output)
    #[allow(missing_docs)]
    BuildFoundryOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "serialNumber")]
        serial_number: u32,
        #[serde(rename = "tokenScheme")]
        token_scheme: TokenSchemeDto,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Build an NftOutput.
    /// Expected response: [`Output`](crate::message_interface::Response::Output)
    #[allow(missing_docs)]
    BuildNftOutput {
        // If not provided, minimum storage deposit will be used
        amount: Option<String>,
        #[serde(rename = "nativeTokens")]
        native_tokens: Option<Vec<NativeTokenDto>>,
        #[serde(rename = "nftId")]
        nft_id: NftIdDto,
        #[serde(rename = "unlockConditions")]
        unlock_conditions: Vec<UnlockConditionDto>,
        features: Option<Vec<FeatureDto>>,
        #[serde(rename = "immutableFeatures")]
        immutable_features: Option<Vec<FeatureDto>>,
    },
    /// Burn native tokens. This doesn't require the foundry output which minted them, but will not increase
    /// the foundries `melted_tokens` field, which makes it impossible to destroy the foundry output. Therefore it's
    /// recommended to use melting, if the foundry output is available.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    BurnNativeToken {
        #[serde(rename = "nativeToken")]
        native_token: NativeTokenDto,
        options: Option<TransactionOptions>,
    },
    /// Burn an nft output. Outputs controlled by it will be sweeped before if they don't have a storage
    /// deposit return, timelock or expiration unlock condition. This should be preferred over burning, because after
    /// burning, the foundry can never be destroyed anymore.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    BurnNft {
        #[serde(rename = "nftId")]
        nft_id: NftIdDto,
        options: Option<TransactionOptions>,
    },
    /// Consolidate outputs.
    /// Expected response: [`SentTransactions`](crate::message_interface::Response::SentTransactions)
    ConsolidateOutputs {
        force: bool,
        #[serde(rename = "outputConsolidationThreshold")]
        output_consolidation_threshold: Option<usize>,
    },
    /// Destroy an alias output. Outputs controlled by it will be sweeped before if they don't have a
    /// storage deposit return, timelock or expiration unlock condition. The amount and possible native tokens will be
    /// sent to the governor address.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    DestroyAlias {
        #[serde(rename = "aliasId")]
        alias_id: AliasIdDto,
        options: Option<TransactionOptions>,
    },
    /// Function to destroy a foundry output with a circulating supply of 0.
    /// Native tokens in the foundry (minted by other foundries) will be transactioned to the controlling alias
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    DestroyFoundry {
        #[serde(rename = "foundryId")]
        foundry_id: FoundryId,
        options: Option<TransactionOptions>,
    },
    /// Generate new unused addresses.
    /// Expected response: [`GeneratedAddress`](crate::message_interface::Response::GeneratedAddress)
    GenerateAddresses {
        amount: u32,
        options: Option<AddressGenerationOptions>,
    },
    /// Get the [`OutputData`](crate::account::types::OutputData) of an output stored in the account
    /// Expected response: [`OutputData`](crate::message_interface::Response::OutputData)
    GetOutput {
        #[serde(rename = "outputId")]
        output_id: OutputId,
    },
    /// Get the [`Output`](crate::account::types::Output) that minted a native token by its TokenId
    /// Expected response: [`Output`](crate::message_interface::Response::Output)
    GetFoundryOutput {
        #[serde(rename = "tokenId")]
        token_id: TokenIdDto,
    },
    /// Get outputs with additional unlock conditions
    /// Expected response: [`OutputIds`](crate::message_interface::Response::OutputIds)
    GetOutputsWithAdditionalUnlockConditions {
        #[serde(rename = "outputsToClaim")]
        outputs_to_claim: OutputsToClaim,
    },
    /// Get the [`Transaction`](crate::account::types::Transaction) of a transaction stored in the account
    /// Expected response: [`Transaction`](crate::message_interface::Response::Transaction)
    GetTransaction {
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },
    /// Get the transaction with inputs of an incoming transaction stored in the account
    /// List might not be complete, if the node pruned the data already
    /// Expected response: [`IncomingTransactionData`](crate::message_interface::Response::IncomingTransactionData)
    GetIncomingTransactionData {
        #[serde(rename = "transactionId")]
        transaction_id: TransactionId,
    },
    /// Expected response: [`Addresses`](crate::message_interface::Response::Addresses)
    /// List addresses.
    ListAddresses,
    /// Returns only addresses of the account with unspent outputs
    /// Expected response:
    /// [`AddressesWithUnspentOutputs`](crate::message_interface::Response::AddressesWithUnspentOutputs)
    ListAddressesWithUnspentOutputs,
    /// Returns all outputs of the account
    /// Expected response: [`OutputsData`](crate::message_interface::Response::OutputsData)
    ListOutputs,
    /// Returns all unspent outputs of the account
    /// Expected response: [`OutputsData`](crate::message_interface::Response::OutputsData)
    ListUnspentOutputs,
    /// Returns all transaction of the account
    /// Expected response: [`Transactions`](crate::message_interface::Response::Transactions)
    ListTransactions,
    /// Returns all pending transaction of the account
    /// Expected response: [`Transactions`](crate::message_interface::Response::Transactions)
    ListPendingTransactions,
    /// Melt native tokens. This happens with the foundry output which minted them, by increasing it's
    /// `melted_tokens` field.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    MeltNativeToken {
        #[serde(rename = "nativeToken")]
        native_token: NativeTokenDto,
        options: Option<TransactionOptions>,
    },
    /// Calculate the minimum required storage deposit for an output.
    /// Expected response:
    /// [`MinimumRequiredStorageDeposit`](crate::message_interface::Response::MinimumRequiredStorageDeposit)
    MinimumRequiredStorageDeposit { output: OutputDto },
    /// Mint native token.
    /// Expected response: [`MintTokenTransaction`](crate::message_interface::Response::MintTokenTransaction)
    MintNativeToken {
        #[serde(rename = "nativeTokenOptions")]
        native_token_options: NativeTokenOptionsDto,
        options: Option<TransactionOptions>,
    },
    /// Mint nft.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    MintNfts {
        #[serde(rename = "nftsOptions")]
        nfts_options: Vec<NftOptionsDto>,
        options: Option<TransactionOptions>,
    },
    /// Get account balance information.
    /// Expected response: [`Balance`](crate::message_interface::Response::Balance)
    GetBalance,
    /// Prepare an output.
    /// Expected response: [`OutputDto`](crate::message_interface::Response::OutputDto)
    PrepareOutput {
        options: OutputOptionsDto,
        transaction_options: Option<TransactionOptions>,
    },
    /// Prepare transaction.
    /// Expected response: [`PreparedTransactionData`](crate::message_interface::Response::PreparedTransactionData)
    PrepareTransaction {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptions>,
    },
    /// Prepare send amount.
    /// Expected response: [`PreparedTransactionData`](crate::message_interface::Response::PreparedTransactionData)
    PrepareSendAmount {
        #[serde(rename = "addressesWithAmount")]
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptions>,
    },
    /// Sync the account by fetching new information from the nodes. Will also retry pending transactions
    /// if necessary.
    /// Expected response: [`Balance`](crate::message_interface::Response::Balance)
    SyncAccount {
        /// Sync options
        options: Option<SyncOptions>,
    },
    /// Send amount.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SendAmount {
        #[serde(rename = "addressesWithAmount")]
        addresses_with_amount: Vec<AddressWithAmountDto>,
        options: Option<TransactionOptions>,
    },
    /// Send amount below minimum storage deposit.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SendMicroTransaction {
        #[serde(rename = "addressesWithMicroAmount")]
        addresses_with_micro_amount: Vec<AddressWithMicroAmountDto>,
        options: Option<TransactionOptions>,
    },
    /// Send native tokens.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SendNativeTokens {
        #[serde(rename = "addressesNativeTokens")]
        addresses_native_tokens: Vec<AddressNativeTokens>,
        options: Option<TransactionOptions>,
    },
    /// Send nft.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SendNft {
        #[serde(rename = "addressesAndNftIds")]
        addresses_nft_ids: Vec<AddressAndNftId>,
        options: Option<TransactionOptions>,
    },
    /// Set the alias of the account.
    /// Expected response: [`Ok`](crate::message_interface::Response::Ok)
    SetAlias { alias: String },
    /// Send outputs in a transaction.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SendOutputs {
        outputs: Vec<OutputDto>,
        options: Option<TransactionOptions>,
    },
    /// Sign a prepared transaction.
    /// Expected response: [`TransactionPayload`](crate::message_interface::Response::TransactionPayload)
    SignTransactionEssence {
        #[serde(rename = "preparedTransactionData")]
        prepared_transaction_data: PreparedTransactionDataDto,
    },
    /// Validate the transaction, submit it to a node and store it in the account.
    /// Expected response: [`SentTransaction`](crate::message_interface::Response::SentTransaction)
    SubmitAndStoreTransaction {
        #[serde(rename = "signedTransactionData")]
        signed_transaction_data: SignedTransactionDataDto,
    },
    /// Try to claim outputs.
    /// Expected response: [`SentTransactions`](crate::message_interface::Response::SentTransactions)
    TryClaimOutputs {
        #[serde(rename = "outputsToClaim")]
        outputs_to_claim: OutputsToClaim,
    },
    /// Claim outputs.
    /// Expected response: [`SentTransactions`](crate::message_interface::Response::SentTransactions)
    ClaimOutputs {
        #[serde(rename = "outputIdsToClaim")]
        output_ids_to_claim: Vec<OutputId>,
    },
}
