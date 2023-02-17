use crate::{self as pallet_chess};
use frame_support::{
    parameter_types,
    traits::{
        AsEnsureOriginWithArg, ConstU16, ConstU32, ConstU64, GenesisBuild, OnFinalize, OnInitialize,
    },
    PalletId,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        Assets: pallet_assets,
        Chess: pallet_chess,
    }
);

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const BulletPeriod: u64 = 10;
    pub const BlitzPeriod: u64 = 50;
    pub const RapidPeriod: u64 = 150;
    pub const DailyPeriod: u64 = 14400;
    pub const ChessPalletId: PalletId = PalletId(*b"subchess");
}

impl pallet_chess::Config for Test {
    type PalletId = ChessPalletId;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_chess::weights::SubstrateWeight<Test>;
    type Assets = Assets;
    type AssetBalance = u64;
    type BulletPeriod = BulletPeriod;
    type BlitzPeriod = BlitzPeriod;
    type RapidPeriod = RapidPeriod;
    type DailyPeriod = DailyPeriod;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
}

impl pallet_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type AssetId = u32;
    type AssetIdParameter = u32;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type AssetDeposit = ConstU64<1>;
    type AssetAccountDeposit = ConstU64<10>;
    type MetadataDepositBase = ConstU64<1>;
    type MetadataDepositPerByte = ConstU64<1>;
    type ApprovalDeposit = ConstU64<1>;
    type StringLimit = ConstU32<50>;
    type Freezer = ();
    type WeightInfo = ();
    type Extra = ();
    type RemoveItemsLimit = ConstU32<5>;
}

frame_support::parameter_types! {
    pub const AssetId: u32 = 200u32;
    pub const AssetMinBalance: u64 = 1_000u64;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    let asset_id = AssetId::get();
    let asset_min_balance = AssetMinBalance::get();

    let config: pallet_assets::GenesisConfig<Test> = pallet_assets::GenesisConfig {
        assets: vec![
            // id, owner, is_sufficient, min_balance
            (asset_id, 0, true, asset_min_balance),
        ],
        metadata: vec![
            // id, name, symbol, decimals
            (asset_id, "Token Name".into(), "TOKEN".into(), 10),
        ],
        accounts: vec![
            // id, account_id, balance
            (asset_id, 1, asset_min_balance * 100),
            (asset_id, 2, asset_min_balance * 100),
        ],
    };
    config.assimilate_storage(&mut storage).unwrap();
    storage.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            Chess::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Chess::on_initialize(System::block_number());
    }
}
