use crate::account::Account;
use bee_crypto::ternary::Kerl;
use bee_signing::ternary::{
  PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsSecurityLevel,
  WotsShakePrivateKeyGeneratorBuilder,
};
pub use bee_transaction::bundled::Address as IotaAddress;
use bee_transaction::bundled::BundledTransactionField;
use getset::Getters;
use serde::{Deserialize, Serialize};

/// The address builder.
#[derive(Default)]
pub struct AddressBuilder {
  address: Option<IotaAddress>,
  balance: Option<u64>,
  key_index: Option<u64>,
  // TODO checksum:
}

impl AddressBuilder {
  /// Initialises a new instance of the address builder.
  pub fn new() -> AddressBuilder {
    Default::default()
  }

  /// Defines the address.
  pub fn address(mut self, address: IotaAddress) -> Self {
    self.address = Some(address);
    self
  }

  /// Sets the address balance.
  pub fn balance(mut self, balance: u64) -> Self {
    self.balance = Some(balance);
    self
  }

  /// Sets the address key index.
  pub fn key_index(mut self, key_index: u64) -> Self {
    self.key_index = Some(key_index);
    self
  }

  /// Builds the address.
  pub fn build(self) -> crate::Result<Address> {
    let address = Address {
      address: self
        .address
        .ok_or_else(|| anyhow::anyhow!("the `address` field is required"))?,
      balance: self
        .balance
        .ok_or_else(|| anyhow::anyhow!("the `balance` field is required"))?,
      key_index: self
        .key_index
        .ok_or_else(|| anyhow::anyhow!("the `key_index` field is required"))?,
    };
    Ok(address)
  }
}

/// An address.
#[derive(Getters, Serialize, Deserialize, Clone)]
#[getset(get = "pub")]
pub struct Address {
  /// The address.
  address: IotaAddress,
  /// The address balance.
  balance: u64,
  /// The address key index.
  key_index: u64,
}

impl PartialEq for Address {
  fn eq(&self, other: &Address) -> bool {
    self.key_index() == other.key_index()
  }
}

/// Gets an unused address for the given account.
pub(crate) fn get_new_address(account: &Account<'_>) -> crate::Result<Address> {
  crate::client::with_client(account.client_options(), |client| {
    let iota_address = client.generate_address().seed(account.seed()).generate()?;
    let address = Address {
      address: iota_address,
      balance: 0,
      key_index: 0,
    };
    Ok(address)
  })
}

/// Batch address generation.
pub(crate) fn get_addresses(account: &Account<'_>, count: u64) -> crate::Result<Vec<Address>> {
  let mut addresses = vec![];
  let seed_trits = account.seed().trits();
  for i in 0..count {
    let address: IotaAddress = IotaAddress::try_from_inner(
      WotsShakePrivateKeyGeneratorBuilder::<Kerl>::default()
        .security_level(WotsSecurityLevel::Medium)
        .build()
        .unwrap()
        .generate_from_entropy(seed_trits)
        .unwrap()
        .generate_public_key()
        .unwrap()
        .to_trits()
        .to_owned(),
    )
    .unwrap();
    addresses.push(Address {
      address,
      balance: 0,
      key_index: i,
    })
  }
  Ok(addresses)
}
