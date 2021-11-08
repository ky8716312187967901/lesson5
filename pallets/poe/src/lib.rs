#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, pallet_prelude::StorageMap, pallet_prelude::*,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    // Learn more about declaring storage items:
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
    pub type Proofs<T: Config> =
        StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRemoved(T::AccountId, Vec<u8>),
        ClaimOwnerChanged(T::AccountId, T::AccountId, Vec<u8>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        ErrorForCreateClaimProofAlreadyExist,
        ErrorForRemoveClaimClaimNotExist,
        ErrorForRemoveClaimUnauthorized,

        ErrorForTransferClaimClaimNotExist,
        ErrorForTransferClaimUnauthorized,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000)]
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let creator = ensure_signed(origin)?;

            ensure!(
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ErrorForCreateClaimProofAlreadyExist
            );

            // insert storage.
            Proofs::<T>::insert(
                &claim,
                (creator.clone(), frame_system::Pallet::<T>::block_number()),
            );
            // Emit an event.
            Self::deposit_event(Event::ClaimCreated(creator, claim));

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn remove_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            let (owner, _) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ErrorForRemoveClaimClaimNotExist)?;

            ensure!(sender == owner, Error::<T>::ErrorForRemoveClaimUnauthorized);
            // Update storage.
            Proofs::<T>::remove(&claim);

            // Emit an event.
            Self::deposit_event(Event::ClaimRemoved(sender, claim));

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            new_owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            let (owner, old_block_number) =
                Proofs::<T>::get(&claim).ok_or(Error::<T>::ErrorForTransferClaimClaimNotExist)?;

            ensure!(
                sender == owner,
                Error::<T>::ErrorForTransferClaimUnauthorized
            );
            // Update storage.
            Proofs::<T>::insert(&claim, (&new_owner, old_block_number));

            // Emit an event.
            Self::deposit_event(Event::ClaimOwnerChanged(owner, new_owner, claim));

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }
    }
}
