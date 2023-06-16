pub mod components;

use crate::AccountInfo;
use crate::U256;
use crate::{Account, Bytecode};
use crate::{B160, B256};
use auto_impl::auto_impl;
use hashbrown::HashMap as Map;

pub use components::{
    BlockHash, BlockHashRef, DatabaseComponentError, DatabaseComponents, State, StateRef,
};

#[async_trait::async_trait]
#[auto_impl(& mut, Box)]
pub trait Database: Send + Sync {
    type Error: Send + Sync;
    /// Get basic account information.
    async fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error>;
    /// Get account code by its hash
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error>;
    /// Get storage value of address at index.
    async fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error>;

    // History related
    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error>;
}

#[auto_impl(& mut, Box)]
pub trait DatabaseCommit {
    fn commit(&mut self, changes: Map<B160, Account>);
}

#[async_trait::async_trait]
#[auto_impl(&, Box, Arc)]
pub trait DatabaseRef: Send + Sync {
    type Error: Send + Sync;
    /// Whether account at address exists.
    //fn exists(&self, address: B160) -> Option<AccountInfo>;
    /// Get basic account information.
    async fn basic(&self, address: B160) -> Result<Option<AccountInfo>, Self::Error>;
    /// Get account code by its hash
    fn code_by_hash(&self, code_hash: B256) -> Result<Bytecode, Self::Error>;
    /// Get storage value of address at index.
    async fn storage(&self, address: B160, index: U256) -> Result<U256, Self::Error>;

    // History related
    fn block_hash(&self, number: U256) -> Result<B256, Self::Error>;
}

pub struct RefDBWrapper<'a, Error: Send + Sync> {
    pub db: &'a dyn DatabaseRef<Error = Error>,
}

impl<'a, Error: Send + Sync> RefDBWrapper<'a, Error> {
    pub fn new(db: &'a dyn DatabaseRef<Error = Error>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl<'a, Error: Send + Sync> Database for RefDBWrapper<'a, Error> {
    type Error = Error;
    /// Get basic account information.
    async fn basic(&mut self, address: B160) -> Result<Option<AccountInfo>, Self::Error> {
        self.db.basic(address).await
    }
    /// Get account code by its hash
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.db.code_by_hash(code_hash)
    }
    /// Get storage value of address at index.
    async fn storage(&mut self, address: B160, index: U256) -> Result<U256, Self::Error> {
        self.db.storage(address, index).await
    }

    // History related
    fn block_hash(&mut self, number: U256) -> Result<B256, Self::Error> {
        self.db.block_hash(number)
    }
}
