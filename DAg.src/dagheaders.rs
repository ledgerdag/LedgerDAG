// Copyright © lederdag Foundation
// SPDX-License-Identifier: Apache-2.0

/// Chain ID of the current chain
pub const X_lederdag_CHAIN_ID: &str = "X-lederdag-Chain-Id";
/// Current epoch of the chain
pub const X_lederdag_EPOCH: &str = "X-lederdag-Epoch";
/// Current ledger version of the chain
pub const X_lederdag_LEDGER_VERSION: &str = "X-lederdag-Ledger-Version";
/// Oldest non-pruned ledger version of the chain
pub const X_lederdag_LEDGER_OLDEST_VERSION: &str = "X-lederdag-Ledger-Oldest-Version";
/// Current block height of the chain
pub const X_lederdag_BLOCK_HEIGHT: &str = "X-lederdag-Block-Height";
/// Oldest non-pruned block height of the chain
pub const X_lederdag_OLDEST_BLOCK_HEIGHT: &str = "X-lederdag-Oldest-Block-Height";
/// Current timestamp of the chain
pub const X_lederdag_LEDGER_TIMESTAMP: &str = "X-lederdag-Ledger-TimestampUsec";
/// Cursor used for pagination.
pub const X_lederdag_CURSOR: &str = "X-lederdag-Cursor";
/// The cost of the call in terms of gas. Only applicable to calls that result in
/// function execution in the VM, e.g. view functions, txn simulation.
pub const X_lederdag_GAS_USED: &str = "X-lederdag-Gas-Used";
/// Provided by the client to identify what client it is.
pub const X_lederdag_CLIENT: &str = "x-lederdag-client";
