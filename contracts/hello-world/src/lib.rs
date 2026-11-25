#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InsufficientPoints = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomerRecord {
    pub balance: i128,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    TotalIssued,
    Customer(Address),
}

#[contract]
pub struct CafePointsContract;

#[contractimpl]
impl CafePointsContract {
    // Khởi tạo contract với địa chỉ chủ quán, chỉ được gọi một lần.
    pub fn init(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::TotalIssued, &0_i128);
        Ok(())
    }

    // Chủ quán cộng điểm cho khách sau lần mua hàng.
    pub fn add_points(env: Env, admin: Address, customer: Address, points: i128) -> Result<i128, Error> {
        admin.require_auth();
        let stored_admin: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;
        if stored_admin != admin {
            return Err(Error::Unauthorized);
        }
        if points <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut record: CustomerRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(customer.clone()))
            .unwrap_or(CustomerRecord {
                balance: 0,
                last_updated: 0,
            });

        record.balance = record
            .balance
            .checked_add(points)
            .ok_or(Error::InvalidAmount)?;
        record.last_updated = env.ledger().timestamp();

        let total_issued: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalIssued)
            .unwrap_or(0);
        let new_total = total_issued
            .checked_add(points)
            .ok_or(Error::InvalidAmount)?;

        env.storage()
            .persistent()
            .set(&DataKey::Customer(customer), &record);
        env.storage()
            .persistent()
            .set(&DataKey::TotalIssued, &new_total);

        Ok(record.balance)
    }

    // Khách tự đổi điểm của mình để nhận ưu đãi.
    pub fn redeem_points(env: Env, customer: Address, points: i128, _reward: String) -> Result<i128, Error> {
        customer.require_auth();
        if !env.storage().persistent().has(&DataKey::Admin) {
            return Err(Error::NotInitialized);
        }
        if points <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut record: CustomerRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(customer.clone()))
            .ok_or(Error::InsufficientPoints)?;

        if record.balance < points {
            return Err(Error::InsufficientPoints);
        }

        record.balance -= points;
        record.last_updated = env.ledger().timestamp();

        env.storage()
            .persistent()
            .set(&DataKey::Customer(customer), &record);

        Ok(record.balance)
    }

    // Xem điểm hiện có của một khách hàng.
    pub fn get_points(env: Env, customer: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Customer(customer))
            .map(|r: CustomerRecord| r.balance)
            .unwrap_or(0)
    }

    // Xem tổng số điểm đã được phát ra từ trước đến nay.
    pub fn get_total_issued(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalIssued)
            .unwrap_or(0)
    }
}