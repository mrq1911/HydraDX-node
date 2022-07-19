use super::*;
use frame_support::pallet_prelude::*;
use hydra_dx_math::omnipool::types::{AssetReserveState as MathReserveState, AssetStateChange, BalanceUpdate};
use scale_info::build::Fields;
use scale_info::{meta_type, Path, Type, TypeParameter};
use sp_runtime::{FixedPointNumber, FixedU128};
use sp_std::ops::{Add, Sub};

/// Balance type used in Omnipool
pub type Balance = u128;

/// Fixed Balance type to represent asset price
pub type Price = FixedU128;

bitflags::bitflags! {
	/// Indicates whether asset can be bought or sold to/from Omnipool and/or liquidity added/removed.
	#[derive(Encode,Decode)]
	pub struct Tradability: u8 {
		/// Asset is frozen. No operations are allowed.
		const FROZEN = 0b0000_0000;
		/// Asset is allowed to be sold into omnipool
		const SELL = 0b0000_0001;
		/// Asset is allowed to be bought into omnipool
		const BUY = 0b0000_0010;
		/// Adding liquidity of asset is allowed
		const ADD_LIQUIDIITY = 0b0000_0100;
		/// Removing liquidity of asset is not allowed
		const REMOVE_LIQUIDITY = 0b0000_1000;
	}
}

impl Default for Tradability {
	fn default() -> Self {
		Tradability::SELL | Tradability::BUY | Tradability::ADD_LIQUIDIITY | Tradability::REMOVE_LIQUIDITY
	}
}

impl MaxEncodedLen for Tradability {
	fn max_encoded_len() -> usize {
		u8::max_encoded_len()
	}
}

impl TypeInfo for Tradability {
	type Identity = Self;

	fn type_info() -> Type {
		Type::builder()
			.path(Path::new("BitFlags", module_path!()))
			.type_params(vec![TypeParameter::new("T", Some(meta_type::<Tradability>()))])
			.composite(Fields::unnamed().field(|f| f.ty::<u64>().type_name("Tradability")))
	}
}

#[test]
fn tradability_should_allow_all_when_default() {
	let default_tradability = Tradability::default();

	assert!(default_tradability.contains(Tradability::BUY));
	assert!(default_tradability.contains(Tradability::SELL));
	assert!(default_tradability.contains(Tradability::ADD_LIQUIDIITY));
	assert!(default_tradability.contains(Tradability::REMOVE_LIQUIDITY));
}

#[derive(Clone, Default, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AssetState<Balance> {
	/// Quantity of Hub Asset matching this asset
	pub(super) hub_reserve: Balance,
	/// Quantity of LP shares for this asset
	pub(super) shares: Balance,
	/// Quantity of LP shares for this asset owned by protocol
	pub(super) protocol_shares: Balance,
	/// TVL of asset
	pub(super) tvl: Balance,
	/// Asset's trade state
	pub(super) tradable: Tradability,
}

impl<Balance> From<AssetReserveState<Balance>> for AssetState<Balance>
where
	Balance: Copy,
{
	fn from(s: AssetReserveState<Balance>) -> Self {
		Self {
			hub_reserve: s.hub_reserve,
			shares: s.shares,
			protocol_shares: s.protocol_shares,
			tvl: s.tvl,
			tradable: s.tradable,
		}
	}
}

/// Position in Omnipool represents a moment when LP provided liquidity of an asset at that moment’s price.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Position<Balance, AssetId> {
	/// Provided Asset
	pub(super) asset_id: AssetId,
	/// Amount of asset added to omnipool
	pub(super) amount: Balance,
	/// Quantity of LP shares owned by LP
	pub(super) shares: Balance,
	/// Price at which liquidity was provided
	// TODO: Due to missing MaxEncodedLen impl for FixedU128, it is not possible to use that type in storage
	// This can change in 0.9.17 where the missing trait is implemented
	// And FixedU128 can be use instead.
	pub(super) price: Balance,
}

impl<Balance, AssetId> From<&Position<Balance, AssetId>> for hydra_dx_math::omnipool::types::Position<Balance>
where
	Balance: Copy + Into<u128>,
{
	fn from(position: &Position<Balance, AssetId>) -> Self {
		Self {
			amount: position.amount,
			shares: position.shares,
			price: FixedU128::from_inner(position.price.into()),
		}
	}
}

impl<Balance, AssetId> Position<Balance, AssetId>
where
	Balance: Into<<FixedU128 as FixedPointNumber>::Inner> + Copy + CheckedAdd + CheckedSub + Default,
{
	/// Update current position state with given delta changes.
	pub(super) fn delta_update(
		self,
		delta_reserve: &BalanceUpdate<Balance>,
		delta_shares: &BalanceUpdate<Balance>,
	) -> Option<Self> {
		Some(Self {
			asset_id: self.asset_id,
			amount: (*delta_reserve + self.amount)?,
			shares: (*delta_shares + self.shares)?,
			price: self.price,
		})
	}
}

/// Simple type to represent imbalance which can be positive or negative.
// Note: Simple prefix is used not to confuse with Imbalance trait from frame_support.
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub(crate) struct SimpleImbalance<Balance> {
	pub value: Balance,
	pub negative: bool,
}

impl<Balance: Default> Default for SimpleImbalance<Balance> {
	fn default() -> Self {
		Self {
			value: Balance::default(),
			negative: true,
		}
	}
}

/// The addition operator + for SimpleImbalance.
///
/// Adds amount to imbalance.
///
/// Note that it returns Option<self> rather than Self.
///
/// Note: Implements `Add` instead of `CheckedAdd` because `CheckedAdd` requires the second parameter
/// to be the same type as the first while we want to add a `Balance` here.
///
/// # Example
///
/// ```ignore
/// let imbalance = SimpleImbalance{value: 100, negative: false} ;
/// assert_eq!(imbalance + 200 , Some(SimpleImbalance{value: 300, negative: false}));
/// ```
impl<Balance: CheckedAdd + CheckedSub + PartialOrd + Copy> Add<Balance> for SimpleImbalance<Balance> {
	type Output = Option<Self>;

	fn add(self, amount: Balance) -> Self::Output {
		let (value, sign) = if !self.negative {
			(self.value.checked_add(&amount)?, self.negative)
		} else if self.value < amount {
			(amount.checked_sub(&self.value)?, false)
		} else {
			(self.value.checked_sub(&amount)?, self.negative)
		};
		Some(Self { value, negative: sign })
	}
}

/// The subtraction operator - for SimpleImbalance.
///
/// Subtracts amount from imbalance.
///
/// Note that it returns Option<self> rather than Self.
///
/// # Example
///
/// ```ignore
/// let imbalance = SimpleImbalance{value: 200, negative: false} ;
/// assert_eq!(imbalance - 100 , Some(SimpleImbalance{value: 100, negative: false}));
/// ```
impl<Balance: CheckedAdd + CheckedSub + PartialOrd + Copy> Sub<Balance> for SimpleImbalance<Balance> {
	type Output = Option<Self>;

	fn sub(self, amount: Balance) -> Self::Output {
		let (value, sign) = if self.negative {
			(self.value.checked_add(&amount)?, self.negative)
		} else if self.value < amount {
			(amount.checked_sub(&self.value)?, true)
		} else {
			(self.value.checked_sub(&amount)?, self.negative)
		};
		Some(Self { value, negative: sign })
	}
}

/// Asset state representation including asset pool reserve.
#[derive(Clone, Default, Debug)]
pub struct AssetReserveState<Balance> {
	/// Quantity of asset in omnipool
	pub(crate) reserve: Balance,
	/// Quantity of Hub Asset matching this asset
	pub(crate) hub_reserve: Balance,
	/// Quantity of LP shares for this asset
	pub(crate) shares: Balance,
	/// Quantity of LP shares for this asset owned by protocol
	pub(crate) protocol_shares: Balance,
	/// TVL of asset
	pub(crate) tvl: Balance,
	/// Asset's trade state
	pub(crate) tradable: Tradability,
}

impl<Balance> From<&AssetReserveState<Balance>> for MathReserveState<Balance>
where
	Balance: Copy,
{
	fn from(state: &AssetReserveState<Balance>) -> Self {
		Self {
			reserve: state.reserve,
			hub_reserve: state.hub_reserve,
			shares: state.shares,
			protocol_shares: state.protocol_shares,
			tvl: state.tvl,
		}
	}
}

impl<Balance> From<(&AssetState<Balance>, Balance)> for AssetReserveState<Balance>
where
	Balance: Copy,
{
	fn from((s, reserve): (&AssetState<Balance>, Balance)) -> Self {
		Self {
			reserve,
			hub_reserve: s.hub_reserve,
			shares: s.shares,
			protocol_shares: s.protocol_shares,
			tvl: s.tvl,
			tradable: s.tradable,
		}
	}
}

impl<Balance> From<(AssetState<Balance>, Balance)> for AssetReserveState<Balance>
where
	Balance: Copy,
{
	fn from((s, reserve): (AssetState<Balance>, Balance)) -> Self {
		Self {
			reserve,
			hub_reserve: s.hub_reserve,
			shares: s.shares,
			protocol_shares: s.protocol_shares,
			tvl: s.tvl,
			tradable: s.tradable,
		}
	}
}

impl<Balance> AssetReserveState<Balance>
where
	Balance: Into<<FixedU128 as FixedPointNumber>::Inner> + Copy + CheckedAdd + CheckedSub + Default,
{
	/// Calculate price for actual state
	pub(crate) fn price(&self) -> Option<FixedU128> {
		FixedU128::checked_from_rational(self.hub_reserve.into(), self.reserve.into())
	}

	/// Update current asset state with given delta changes.
	pub(crate) fn delta_update(self, delta: &AssetStateChange<Balance>) -> Option<Self> {
		Some(Self {
			reserve: (delta.delta_reserve + self.reserve)?,
			hub_reserve: (delta.delta_hub_reserve + self.hub_reserve)?,
			shares: (delta.delta_shares + self.shares)?,
			protocol_shares: (delta.delta_protocol_shares + self.protocol_shares)?,
			tvl: (delta.delta_tvl + self.tvl)?,
			tradable: self.tradable,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::SimpleImbalance;
	#[test]
	fn simple_imbalance_addition_works() {
		assert_eq!(
			SimpleImbalance {
				value: 100,
				negative: false
			} + 200,
			Some(SimpleImbalance {
				value: 300,
				negative: false
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 100,
				negative: true
			} + 200,
			Some(SimpleImbalance {
				value: 100,
				negative: false
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 500,
				negative: true
			} + 200,
			Some(SimpleImbalance {
				value: 300,
				negative: true
			})
		);

		assert_eq!(
			SimpleImbalance {
				value: 500,
				negative: true
			} + 500,
			Some(SimpleImbalance {
				value: 0,
				negative: true
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 0,
				negative: true
			} + 500,
			Some(SimpleImbalance {
				value: 500,
				negative: false
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 0,
				negative: false
			} + 500,
			Some(SimpleImbalance {
				value: 500,
				negative: false
			})
		);

		assert_eq!(
			SimpleImbalance {
				value: 1u128,
				negative: true
			} + u128::MAX,
			Some(SimpleImbalance {
				value: u128::MAX - 1,
				negative: false
			})
		);

		assert_eq!(
			SimpleImbalance {
				value: u128::MAX,
				negative: false
			} + 1,
			None
		);
		assert_eq!(
			SimpleImbalance {
				value: 1u128,
				negative: false
			} + u128::MAX,
			None
		);
	}

	#[test]
	fn simple_imbalance_subtraction_works() {
		assert_eq!(
			SimpleImbalance {
				value: 200,
				negative: false
			} - 300,
			Some(SimpleImbalance {
				value: 100,
				negative: true
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 200,
				negative: true
			} - 300,
			Some(SimpleImbalance {
				value: 500,
				negative: true
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 300,
				negative: false
			} - 300,
			Some(SimpleImbalance {
				value: 0,
				negative: false
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 0,
				negative: false
			} - 300,
			Some(SimpleImbalance {
				value: 300,
				negative: true
			})
		);
		assert_eq!(
			SimpleImbalance {
				value: 0,
				negative: true
			} - 300,
			Some(SimpleImbalance {
				value: 300,
				negative: true
			})
		);

		assert_eq!(
			SimpleImbalance {
				value: 1u128,
				negative: false
			} - u128::MAX,
			Some(SimpleImbalance {
				value: u128::MAX - 1,
				negative: true
			})
		);

		assert_eq!(
			SimpleImbalance {
				value: u128::MAX,
				negative: true
			} - 1,
			None
		);
		assert_eq!(
			SimpleImbalance {
				value: 1u128,
				negative: true
			} - u128::MAX,
			None
		);
	}
}
