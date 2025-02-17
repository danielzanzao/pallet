use frame_support::construct_runtime;
use frame_system as system;
use pallet_biblioteca;

impl pallet_biblioteca::Config for Runtime {
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{Pallet, Call, Storage, Config, Event<T>},
        PalletBiblioteca: pallet_biblioteca::{Pallet, Call, Storage, Event<T>},
    }
);
