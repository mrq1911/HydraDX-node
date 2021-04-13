pub mod currency {
	pub use crate::Balance;

	pub const HDX: Balance = 1_000_000_000_000;
	pub const DOLLARS: Balance = HDX * 10; // 10 HDX ~= 1 $
	pub const CENTS: Balance = DOLLARS / 100;
	pub const MILLICENTS: Balance = CENTS / 1_000;

	pub const FORTUNE: Balance = u128::MAX;
}

pub mod time {
	use crate::{BlockNumber, Moment};

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targeting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>

	pub const MILLISECS_PER_BLOCK: Moment = 6_000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 4 * HOURS;
	pub const EPOCH_DURATION_IN_SLOTS: u64 = {
		const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

		(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
	};
	pub const INFINITY: u32 = u32::MAX;
}

pub mod chain {
	pub use crate::{AssetId, Balance};
	pub use frame_support::weights::{constants::WEIGHT_PER_SECOND, Weight};
	pub use sp_runtime::Perbill;

	/// Core asset id
	pub const CORE_ASSET_ID: AssetId = 0;

	/// Max fraction of pool to buy in single transaction
	pub const MAX_OUT_RATIO: u128 = 3;

	/// Max fraction of pool to sell in single transaction
	pub const MAX_IN_RATIO: u128 = 3;

	/// Trading limit
	pub const MIN_TRADING_LIMIT: Balance = 1000;

	pub const RUNTIME_AUTHORING_VERSION: u32 = 1;
	pub const RUNTIME_SPEC_VERSION: u32 = 7;
	pub const RUNTIME_IMPL_VERSION: u32 = 0;
	pub const RUNTIME_TRANSACTION_VERSION: u32 = 1;

	/// We assume that an on-initialize consumes 2.5% of the weight on average, hence a single extrinsic
	/// will not be allowed to consume more than `AvailableBlockRatio - 2.5%`.
	pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_perthousand(25);
	/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
	/// by  Operational  extrinsics.
	pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
	/// We allow for 2 seconds of compute with a 6 second average block time.
	pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;
}
