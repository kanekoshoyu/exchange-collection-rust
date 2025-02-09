#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsDelta represents a union of types: WsDeposit, WsWithdraw, WsInternalTransfer, WsSubAccountTransfer, WsLedgerLiquidation, WsVaultDelta, WsVaultWithdrawal, WsVaultLeaderCommission
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum WsDelta {
    #[serde(rename="WsDeposit")]
    WsDeposit(WsDeposit),
    #[serde(rename="WsWithdraw")]
    WsWithdraw(WsWithdraw),
    #[serde(rename="WsInternalTransfer")]
    WsInternalTransfer(WsInternalTransfer),
    #[serde(rename="WsSubAccountTransfer")]
    WsSubAccountTransfer(WsSubAccountTransfer),
    #[serde(rename="WsLedgerLiquidation")]
    WsLedgerLiquidation(WsLedgerLiquidation),
    #[serde(rename="WsVaultDelta")]
    WsVaultDelta(WsVaultDelta),
    #[serde(rename="WsVaultWithdrawal")]
    WsVaultWithdrawal(WsVaultWithdrawal),
    #[serde(rename="WsVaultLeaderCommission")]
    WsVaultLeaderCommission(WsVaultLeaderCommission),
}


