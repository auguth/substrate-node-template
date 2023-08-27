use crate::{
	exec::{AccountIdOf, Key},
	weights::WeightInfo,
	AddressGenerator, BalanceOf, CodeHash, Config, ContractInfoOf, DeletionQueue,
	DeletionQueueCounter, Error, Pallet, TrieId, SENTINEL,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::DispatchError,
	storage::child::{self, ChildInfo},
	weights::Weight,
	DefaultNoBound, RuntimeDebugNoBound,
};
use scale_info::TypeInfo;
use sp_io::KillStorageResult;
use sp_runtime::{
	traits::{Hash, Saturating, Zero},
	RuntimeDebug,
};
use sp_std::{marker::PhantomData, ops::Deref, prelude::*};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub struct Stakeinfo<T: Config> {
	pub owner : T::AccountId,
	pub delegate_to: T::AccountId,
	pub delegate_at: BlockNumberFor<T>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]

pub struct ContractScarcityInfo<T: Config> {
	pub contract_address : T::AccountId,
	pub reputation: u64,
	pub weight: Weight,
}

impl<T: Config> Stakeinfo<T> {

    pub fn set_new_stakeinfo(
		owner: T::AccountId,
        delegate_to: T::AccountId,
		delegate_at: BlockNumberFor<T>,
	) -> Result<Self,Error<T>>{

		let info = Self {
			owner,
            delegate_to,
			delegate_at,
		};

		Ok(info)
	}

}
