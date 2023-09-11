use crate::{
	weights::WeightInfo,
	AddressGenerator, BalanceOf, CodeHash, Config, ContractInfoOf, Error, Pallet, TrieId, SENTINEL, Pallet as Contracts,Event,
};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::DispatchError,
	storage::StorageMap,
	pallet_prelude::GetDefault,
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
pub struct AccountStakeinfo<T: Config> {
	pub owner : T::AccountId,
	pub delegate_to: T::AccountId,
	pub delegate_at: BlockNumberFor<T>,
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct ContractScarcityInfo<T: Config> {
	pub reputation: u64,
	pub weight_history: Weight,
	pub recent_blockhight: BlockNumberFor<T>,
}

impl<T: Config> AccountStakeinfo<T> {

    pub fn set_new_stakeinfo(
		owner: T::AccountId,
        delegate_to: T::AccountId,
	) -> Self{
		let current_block_number = <frame_system::Pallet<T>>::block_number();

		Self {
			owner,
            delegate_to,
			delegate_at:current_block_number,
		}
	}

}

impl<T: Config> ContractScarcityInfo<T>{

	pub fn set_scarcity_info()->Self{

		let current_block_number = <frame_system::Pallet<T>>::block_number();
		Self{
			reputation: 1,
	        weight_history: Weight::zero(),
			recent_blockhight: current_block_number,
		}
	}

	pub fn update_scarcity_info(
		current_reputation: u64,
		new_weight_history: Weight,
		old_block_hight: BlockNumberFor<T>,
	)-> Self{

		let current_block_hight = <frame_system::Pallet<T>>::block_number();

		if current_block_hight > old_block_hight{
		let new_reputation = current_reputation + 10;
		let new_recent_blockhight = current_block_hight;

		Self{

			reputation: new_reputation,
			weight_history: new_weight_history,
			recent_blockhight: new_recent_blockhight,
		}
		}
		else{
		 let new_reputation = current_reputation;
		 let new_recent_blockhight = old_block_hight;

		 Self{

			reputation: new_reputation,
			weight_history: new_weight_history,
			recent_blockhight: new_recent_blockhight,
		}
		}
		
	}


}