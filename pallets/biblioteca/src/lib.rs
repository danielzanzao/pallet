#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Genero {
        Ficcao,
        Biografia,
        Poesia,
        Infantil,
        Romance,
        Outro,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Livro<AccountId> {
        id: u32,
        titulo: BoundedVec<u8, ConstU32<100>>,
        genero: Genero,
        dono: AccountId,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub(super) type Livros<T: Config> = StorageMap<_, Blake2_128Concat, u32, Livro<T::AccountId>, OptionQuery>;

    #[pallet::storage]
    pub(super) type ProximoId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn adicionar_livro(origin: OriginFor<T>, titulo: BoundedVec<u8, ConstU32<100>>, genero: Genero) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let id = ProximoId::<T>::get();

            let livro = Livro { id, titulo, genero, dono: sender.clone() };
            Livros::<T>::insert(id, livro);
            ProximoId::<T>::put(id + 1);

            Ok(())
        }
    }
}
