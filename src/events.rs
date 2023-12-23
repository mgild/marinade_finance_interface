use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const CHANGE_AUTHORITY_EVENT_EVENT_DISCM: [u8; 8] = [228, 111, 35, 24, 187, 78, 224, 138];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ChangeAuthorityEvent {
    state: Pubkey,
    admin_change: Option<PubkeyValueChange>,
    validator_manager_change: Option<PubkeyValueChange>,
    operational_sol_account_change: Option<PubkeyValueChange>,
    treasury_msol_account_change: Option<PubkeyValueChange>,
    pause_authority_change: Option<PubkeyValueChange>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeAuthorityEventEvent(pub ChangeAuthorityEvent);
impl BorshSerialize for ChangeAuthorityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CHANGE_AUTHORITY_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ChangeAuthorityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CHANGE_AUTHORITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CHANGE_AUTHORITY_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ChangeAuthorityEvent::deserialize(buf)?))
    }
}
pub const CONFIG_LP_EVENT_EVENT_DISCM: [u8; 8] = [159, 204, 192, 138, 68, 145, 224, 148];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ConfigLpEvent {
    state: Pubkey,
    min_fee_change: Option<FeeValueChange>,
    max_fee_change: Option<FeeValueChange>,
    liquidity_target_change: Option<U64ValueChange>,
    treasury_cut_change: Option<FeeValueChange>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigLpEventEvent(pub ConfigLpEvent);
impl BorshSerialize for ConfigLpEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CONFIG_LP_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ConfigLpEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_LP_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_LP_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigLpEvent::deserialize(buf)?))
    }
}
pub const CONFIG_MARINADE_EVENT_EVENT_DISCM: [u8; 8] = [159, 164, 245, 114, 94, 253, 3, 9];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ConfigMarinadeEvent {
    state: Pubkey,
    rewards_fee_change: Option<FeeValueChange>,
    slots_for_stake_delta_change: Option<U64ValueChange>,
    min_stake_change: Option<U64ValueChange>,
    min_deposit_change: Option<U64ValueChange>,
    min_withdraw_change: Option<U64ValueChange>,
    staking_sol_cap_change: Option<U64ValueChange>,
    liquidity_sol_cap_change: Option<U64ValueChange>,
    withdraw_stake_account_enabled_change: Option<BoolValueChange>,
    delayed_unstake_fee_change: Option<FeeCentsValueChange>,
    withdraw_stake_account_fee_change: Option<FeeCentsValueChange>,
    max_stake_moved_per_epoch_change: Option<FeeValueChange>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigMarinadeEventEvent(pub ConfigMarinadeEvent);
impl BorshSerialize for ConfigMarinadeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CONFIG_MARINADE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ConfigMarinadeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_MARINADE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONFIG_MARINADE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConfigMarinadeEvent::deserialize(buf)?))
    }
}
pub const INITIALIZE_EVENT_EVENT_DISCM: [u8; 8] = [206, 175, 169, 208, 241, 210, 35, 221];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct InitializeEvent {
    state: Pubkey,
    params: InitializeData,
    stake_list: Pubkey,
    validator_list: Pubkey,
    msol_mint: Pubkey,
    operational_sol_account: Pubkey,
    lp_mint: Pubkey,
    lp_msol_leg: Pubkey,
    treasury_msol_account: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeEventEvent(pub InitializeEvent);
impl BorshSerialize for InitializeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        INITIALIZE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl InitializeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != INITIALIZE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    INITIALIZE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(InitializeEvent::deserialize(buf)?))
    }
}
pub const EMERGENCY_PAUSE_EVENT_EVENT_DISCM: [u8; 8] = [159, 241, 192, 232, 29, 208, 51, 21];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct EmergencyPauseEvent {
    state: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmergencyPauseEventEvent(pub EmergencyPauseEvent);
impl BorshSerialize for EmergencyPauseEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        EMERGENCY_PAUSE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl EmergencyPauseEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != EMERGENCY_PAUSE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    EMERGENCY_PAUSE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(EmergencyPauseEvent::deserialize(buf)?))
    }
}
pub const RESUME_EVENT_EVENT_DISCM: [u8; 8] = [97, 117, 183, 115, 117, 224, 8, 229];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ResumeEvent {
    state: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ResumeEventEvent(pub ResumeEvent);
impl BorshSerialize for ResumeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        RESUME_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ResumeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != RESUME_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    RESUME_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ResumeEvent::deserialize(buf)?))
    }
}
pub const REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM: [u8; 8] = [70, 191, 242, 164, 56, 156, 130, 13];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ReallocValidatorListEvent {
    state: Pubkey,
    count: u32,
    new_capacity: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocValidatorListEventEvent(pub ReallocValidatorListEvent);
impl BorshSerialize for ReallocValidatorListEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ReallocValidatorListEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REALLOC_VALIDATOR_LIST_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReallocValidatorListEvent::deserialize(buf)?))
    }
}
pub const REALLOC_STAKE_LIST_EVENT_EVENT_DISCM: [u8; 8] = [193, 129, 16, 243, 177, 131, 248, 23];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ReallocStakeListEvent {
    state: Pubkey,
    count: u32,
    new_capacity: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReallocStakeListEventEvent(pub ReallocStakeListEvent);
impl BorshSerialize for ReallocStakeListEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REALLOC_STAKE_LIST_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ReallocStakeListEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REALLOC_STAKE_LIST_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REALLOC_STAKE_LIST_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ReallocStakeListEvent::deserialize(buf)?))
    }
}
pub const DEACTIVATE_STAKE_EVENT_EVENT_DISCM: [u8; 8] = [2, 54, 184, 218, 78, 181, 163, 117];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DeactivateStakeEvent {
    state: Pubkey,
    epoch: u64,
    stake_index: u32,
    stake_account: Pubkey,
    last_update_stake_delegation: u64,
    split_stake_account: Option<SplitStakeAccountInfo>,
    validator_index: u32,
    validator_vote: Pubkey,
    total_stake_target: u64,
    validator_stake_target: u64,
    total_active_balance: u64,
    delayed_unstake_cooling_down: u64,
    validator_active_balance: u64,
    total_unstake_delta: u64,
    unstaked_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeactivateStakeEventEvent(pub DeactivateStakeEvent);
impl BorshSerialize for DeactivateStakeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DEACTIVATE_STAKE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl DeactivateStakeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEACTIVATE_STAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEACTIVATE_STAKE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeactivateStakeEvent::deserialize(buf)?))
    }
}
pub const MERGE_STAKES_EVENT_EVENT_DISCM: [u8; 8] = [73, 156, 69, 233, 32, 14, 150, 65];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct MergeStakesEvent {
    state: Pubkey,
    epoch: u64,
    destination_stake_index: u32,
    destination_stake_account: Pubkey,
    last_update_destination_stake_delegation: u64,
    source_stake_index: u32,
    source_stake_account: Pubkey,
    last_update_source_stake_delegation: u64,
    validator_index: u32,
    validator_vote: Pubkey,
    extra_delegated: u64,
    returned_stake_rent: u64,
    validator_active_balance: u64,
    total_active_balance: u64,
    operational_sol_balance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MergeStakesEventEvent(pub MergeStakesEvent);
impl BorshSerialize for MergeStakesEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        MERGE_STAKES_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl MergeStakesEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != MERGE_STAKES_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MERGE_STAKES_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(MergeStakesEvent::deserialize(buf)?))
    }
}
pub const REDELEGATE_EVENT_EVENT_DISCM: [u8; 8] = [241, 75, 135, 173, 204, 215, 72, 67];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RedelegateEvent {
    state: Pubkey,
    epoch: u64,
    stake_index: u32,
    stake_account: Pubkey,
    last_update_delegation: u64,
    source_validator_index: u32,
    source_validator_vote: Pubkey,
    source_validator_score: u32,
    source_validator_balance: u64,
    source_validator_stake_target: u64,
    dest_validator_index: u32,
    dest_validator_vote: Pubkey,
    dest_validator_score: u32,
    dest_validator_balance: u64,
    dest_validator_stake_target: u64,
    redelegate_amount: u64,
    split_stake_account: Option<SplitStakeAccountInfo>,
    redelegate_stake_index: u32,
    redelegate_stake_account: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RedelegateEventEvent(pub RedelegateEvent);
impl BorshSerialize for RedelegateEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REDELEGATE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl RedelegateEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REDELEGATE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REDELEGATE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RedelegateEvent::deserialize(buf)?))
    }
}
pub const STAKE_RESERVE_EVENT_EVENT_DISCM: [u8; 8] = [112, 117, 149, 185, 77, 119, 190, 106];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct StakeReserveEvent {
    state: Pubkey,
    epoch: u64,
    stake_index: u32,
    stake_account: Pubkey,
    validator_index: u32,
    validator_vote: Pubkey,
    total_stake_target: u64,
    validator_stake_target: u64,
    reserve_balance: u64,
    total_active_balance: u64,
    validator_active_balance: u64,
    total_stake_delta: u64,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeReserveEventEvent(pub StakeReserveEvent);
impl BorshSerialize for StakeReserveEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        STAKE_RESERVE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl StakeReserveEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != STAKE_RESERVE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    STAKE_RESERVE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(StakeReserveEvent::deserialize(buf)?))
    }
}
pub const UPDATE_ACTIVE_EVENT_EVENT_DISCM: [u8; 8] = [251, 18, 128, 75, 208, 80, 174, 140];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateActiveEvent {
    state: Pubkey,
    epoch: u64,
    stake_index: u32,
    stake_account: Pubkey,
    validator_index: u32,
    validator_vote: Pubkey,
    delegation_change: U64ValueChange,
    delegation_growth_msol_fees: Option<u64>,
    extra_lamports: u64,
    extra_msol_fees: Option<u64>,
    validator_active_balance: u64,
    total_active_balance: u64,
    msol_price_change: U64ValueChange,
    reward_fee_used: Fee,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateActiveEventEvent(pub UpdateActiveEvent);
impl BorshSerialize for UpdateActiveEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_ACTIVE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateActiveEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_ACTIVE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_ACTIVE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateActiveEvent::deserialize(buf)?))
    }
}
pub const UPDATE_DEACTIVATED_EVENT_EVENT_DISCM: [u8; 8] = [252, 159, 177, 147, 182, 113, 186, 94];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateDeactivatedEvent {
    state: Pubkey,
    epoch: u64,
    stake_index: u32,
    stake_account: Pubkey,
    balance_without_rent_exempt: u64,
    last_update_delegated_lamports: u64,
    msol_fees: Option<u64>,
    msol_price_change: U64ValueChange,
    reward_fee_used: Fee,
    operational_sol_balance: u64,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateDeactivatedEventEvent(pub UpdateDeactivatedEvent);
impl BorshSerialize for UpdateDeactivatedEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_DEACTIVATED_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateDeactivatedEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_DEACTIVATED_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_DEACTIVATED_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateDeactivatedEvent::deserialize(buf)?))
    }
}
pub const CLAIM_EVENT_EVENT_DISCM: [u8; 8] = [93, 15, 70, 170, 48, 140, 212, 219];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimEvent {
    state: Pubkey,
    epoch: u64,
    ticket: Pubkey,
    beneficiary: Pubkey,
    circulating_ticket_balance: u64,
    circulating_ticket_count: u64,
    reserve_balance: u64,
    user_balance: u64,
    amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimEventEvent(pub ClaimEvent);
impl BorshSerialize for ClaimEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CLAIM_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ClaimEvent::deserialize(buf)?))
    }
}
pub const ORDER_UNSTAKE_EVENT_EVENT_DISCM: [u8; 8] = [228, 63, 155, 249, 132, 160, 135, 113];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct OrderUnstakeEvent {
    state: Pubkey,
    ticket_epoch: u64,
    ticket: Pubkey,
    beneficiary: Pubkey,
    circulating_ticket_balance: u64,
    circulating_ticket_count: u64,
    user_msol_balance: u64,
    burned_msol_amount: u64,
    sol_amount: u64,
    fee_bp_cents: u32,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct OrderUnstakeEventEvent(pub OrderUnstakeEvent);
impl BorshSerialize for OrderUnstakeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ORDER_UNSTAKE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl OrderUnstakeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ORDER_UNSTAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ORDER_UNSTAKE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(OrderUnstakeEvent::deserialize(buf)?))
    }
}
pub const ADD_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [27, 178, 153, 186, 47, 196, 140, 45];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AddLiquidityEvent {
    state: Pubkey,
    sol_owner: Pubkey,
    user_sol_balance: u64,
    user_lp_balance: u64,
    sol_leg_balance: u64,
    lp_supply: u64,
    sol_added_amount: u64,
    lp_minted: u64,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddLiquidityEventEvent(pub AddLiquidityEvent);
impl BorshSerialize for AddLiquidityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADD_LIQUIDITY_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AddLiquidityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_LIQUIDITY_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddLiquidityEvent::deserialize(buf)?))
    }
}
pub const LIQUID_UNSTAKE_EVENT_EVENT_DISCM: [u8; 8] = [173, 5, 147, 15, 5, 14, 194, 116];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LiquidUnstakeEvent {
    state: Pubkey,
    msol_owner: Pubkey,
    liq_pool_sol_balance: u64,
    liq_pool_msol_balance: u64,
    treasury_msol_balance: Option<u64>,
    user_msol_balance: u64,
    user_sol_balance: u64,
    msol_amount: u64,
    msol_fee: u64,
    treasury_msol_cut: u64,
    sol_amount: u64,
    lp_liquidity_target: u64,
    lp_max_fee: Fee,
    lp_min_fee: Fee,
    treasury_cut: Fee,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LiquidUnstakeEventEvent(pub LiquidUnstakeEvent);
impl BorshSerialize for LiquidUnstakeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LIQUID_UNSTAKE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LiquidUnstakeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LIQUID_UNSTAKE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    LIQUID_UNSTAKE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(LiquidUnstakeEvent::deserialize(buf)?))
    }
}
pub const REMOVE_LIQUIDITY_EVENT_EVENT_DISCM: [u8; 8] = [141, 199, 182, 123, 159, 94, 215, 102];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RemoveLiquidityEvent {
    state: Pubkey,
    sol_leg_balance: u64,
    msol_leg_balance: u64,
    user_lp_balance: u64,
    user_sol_balance: u64,
    user_msol_balance: u64,
    lp_mint_supply: u64,
    lp_burned: u64,
    sol_out_amount: u64,
    msol_out_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveLiquidityEventEvent(pub RemoveLiquidityEvent);
impl BorshSerialize for RemoveLiquidityEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REMOVE_LIQUIDITY_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl RemoveLiquidityEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_LIQUIDITY_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_LIQUIDITY_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveLiquidityEvent::deserialize(buf)?))
    }
}
pub const ADD_VALIDATOR_EVENT_EVENT_DISCM: [u8; 8] = [190, 231, 170, 244, 14, 227, 129, 66];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AddValidatorEvent {
    state: Pubkey,
    validator: Pubkey,
    index: u32,
    score: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddValidatorEventEvent(pub AddValidatorEvent);
impl BorshSerialize for AddValidatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADD_VALIDATOR_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AddValidatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_VALIDATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    ADD_VALIDATOR_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(AddValidatorEvent::deserialize(buf)?))
    }
}
pub const REMOVE_VALIDATOR_EVENT_EVENT_DISCM: [u8; 8] = [67, 164, 190, 192, 156, 156, 168, 210];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RemoveValidatorEvent {
    state: Pubkey,
    validator: Pubkey,
    index: u32,
    operational_sol_balance: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveValidatorEventEvent(pub RemoveValidatorEvent);
impl BorshSerialize for RemoveValidatorEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REMOVE_VALIDATOR_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl RemoveValidatorEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_VALIDATOR_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_VALIDATOR_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveValidatorEvent::deserialize(buf)?))
    }
}
pub const SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM: [u8; 8] = [58, 53, 237, 178, 238, 153, 85, 156];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SetValidatorScoreEvent {
    state: Pubkey,
    validator: Pubkey,
    index: u32,
    score_change: U32ValueChange,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SetValidatorScoreEventEvent(pub SetValidatorScoreEvent);
impl BorshSerialize for SetValidatorScoreEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SetValidatorScoreEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_VALIDATOR_SCORE_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetValidatorScoreEvent::deserialize(buf)?))
    }
}
pub const DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [231, 203, 118, 96, 75, 116, 70, 228];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DepositStakeAccountEvent {
    state: Pubkey,
    stake: Pubkey,
    delegated: u64,
    withdrawer: Pubkey,
    stake_index: u32,
    validator: Pubkey,
    validator_index: u32,
    validator_active_balance: u64,
    total_active_balance: u64,
    user_msol_balance: u64,
    msol_minted: u64,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositStakeAccountEventEvent(pub DepositStakeAccountEvent);
impl BorshSerialize for DepositStakeAccountEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl DepositStakeAccountEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_STAKE_ACCOUNT_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositStakeAccountEvent::deserialize(buf)?))
    }
}
pub const DEPOSIT_EVENT_EVENT_DISCM: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct DepositEvent {
    state: Pubkey,
    sol_owner: Pubkey,
    user_sol_balance: u64,
    user_msol_balance: u64,
    sol_leg_balance: u64,
    msol_leg_balance: u64,
    reserve_balance: u64,
    sol_swapped: u64,
    msol_swapped: u64,
    sol_deposited: u64,
    msol_minted: u64,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DepositEventEvent(pub DepositEvent);
impl BorshSerialize for DepositEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        DEPOSIT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl DepositEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != DEPOSIT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPOSIT_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DepositEvent::deserialize(buf)?))
    }
}
pub const WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM: [u8; 8] = [131, 238, 39, 48, 30, 27, 165, 28];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawStakeAccountEvent {
    state: Pubkey,
    epoch: u64,
    stake: Pubkey,
    last_update_stake_delegation: u64,
    stake_index: u32,
    validator: Pubkey,
    validator_index: u32,
    user_msol_balance: u64,
    user_msol_auth: Pubkey,
    msol_burned: u64,
    msol_fees: u64,
    split_stake: Pubkey,
    beneficiary: Pubkey,
    split_lamports: u64,
    fee_bp_cents: u32,
    total_virtual_staked_lamports: u64,
    msol_supply: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawStakeAccountEventEvent(pub WithdrawStakeAccountEvent);
impl BorshSerialize for WithdrawStakeAccountEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WithdrawStakeAccountEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    WITHDRAW_STAKE_ACCOUNT_EVENT_EVENT_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(WithdrawStakeAccountEvent::deserialize(buf)?))
    }
}
