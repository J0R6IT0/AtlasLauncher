use std::{cmp::Ordering, sync::Mutex};

use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

use crate::{
    auth::msauth::auth_flow,
    util::file::{read_to_value, write_value},
};

/// Represents a Minecraft account.
#[derive(Clone, Serialize, Deserialize)]
pub struct MinecraftAccount {
    pub username: String,
    pub uuid: String,
    pub refresh_token: String,
    pub access_token: String,
    pub active: bool,
    pub avatar_64: String,
}

impl Ord for MinecraftAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl PartialOrd for MinecraftAccount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MinecraftAccount {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for MinecraftAccount {}

/// A `state` to manage Minecraft accounts.
///
/// This is meant to be used with [`tauri`](tauri)'s state manager.
pub struct AccountManager {
    accounts: Mutex<Vec<MinecraftAccount>>,
}

impl AccountManager {
    /// Initializes the state by reading the accounts file.
    pub async fn init() -> Self {
        let accounts: Vec<MinecraftAccount> = read_to_value("launcher/accounts.json".to_string())
            .await
            .unwrap_or_default();

        Self {
            accounts: Mutex::new(accounts),
        }
    }

    /// Returns the current accounts
    pub fn accounts(&self) -> Vec<MinecraftAccount> {
        let mutex_lock = self.accounts.lock().unwrap();
        mutex_lock.clone()
    }

    /// Removes a specific account
    pub async fn remove_account(&self, uuid: &str) -> Result<(), &'static str> {
        {
            let mut mutex_lock = self.accounts.lock().unwrap();
            mutex_lock.retain(|acc| acc.uuid != uuid);
        }

        self.save_acounts().await?;
        Ok(())
    }

    /// Refreshes the data (tokens, info) of every account.
    pub async fn refresh_accounts(&self) -> Result<(), &'static str> {
        let mut join_set = JoinSet::new();

        {
            let mutex_lock = self.accounts.lock().unwrap();

            for account in mutex_lock.iter() {
                let refresh_token = account.refresh_token.clone();

                join_set.spawn(async move { auth_flow(&refresh_token, true).await });
            }
        }

        while join_set.join_next().await.is_some() {}

        Ok(())
    }

    /// Adds a new user.
    pub async fn add_account(
        &self,
        account: MinecraftAccount,
        force_active: bool,
    ) -> Result<(), &'static str> {
        let account_uuid = account.uuid.clone();
        let set_active = force_active || self.is_account_active(&account_uuid);
        {
            let mut mutex_lock = self.accounts.lock().unwrap();

            mutex_lock.retain(|acc| acc.uuid != account_uuid);
            mutex_lock.push(account);
        }
        if set_active {
            self.set_active_account(&account_uuid).await?;
        }
        self.save_acounts().await?;
        Ok(())
    }

    /// Returs a specific account
    fn get_account(&self, uuid: &str) -> Option<MinecraftAccount> {
        let mutex_lock = self.accounts.lock().unwrap();
        mutex_lock
            .iter()
            .find(|acc| acc.uuid == uuid)
            .map(|acc| acc.to_owned())
    }

    /// Makes an account active.
    pub async fn set_active_account(&self, uuid: &str) -> Result<(), &'static str> {
        {
            let mut mutex_lock = self.accounts.lock().unwrap();
            for account in &mut mutex_lock.iter_mut() {
                account.active = account.uuid == uuid;
            }
        }
        self.save_acounts().await?;

        Ok(())
    }

    /// Checks if an account is active
    pub fn is_account_active(&self, uuid: &str) -> bool {
        let account = self.get_account(uuid);
        match account {
            Some(acc) => acc.active,
            None => false,
        }
    }

    /// Saves the current accounts to file
    async fn save_acounts(&self) -> Result<(), &'static str> {
        write_value(&self.accounts(), "launcher/accounts.json").await?;
        Ok(())
    }
}
