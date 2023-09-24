use crate::{Config,BalanceOf};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	weights::Weight,	
};

use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Hash, Saturating, Zero},
	RuntimeDebug,AccountId32,
};
use sp_std::{marker::PhantomData, ops::Deref, prelude::*};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct ValidatorStakeinfo<T: Config> {
	pub validator: T::AccountId,
	pub score: BalanceOf<T>,
}

impl<T: Config> ValidatorStakeinfo<T> {

    pub fn set_new_stakeinfo(
		validator: T::AccountId,
        score: BalanceOf<T>,
	) -> Self{

		Self {
			validator,
			score,
		}
	}

}
