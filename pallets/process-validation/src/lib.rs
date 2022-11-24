#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, Parameter, RuntimeDebug};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::traits::{AtLeast32Bit, One};
use sp_std::prelude::*;

use dscp_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator};

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// import the restrictions module where all our restriction types are defined
mod restrictions;
pub use restrictions::*;

mod binary_expression_tree;
pub use binary_expression_tree::*;

#[derive(Encode, Debug, Decode, Clone, MaxEncodedLen, TypeInfo, PartialEq)]
pub enum ProcessStatus {
    Disabled,
    Enabled
}

impl Default for ProcessStatus {
    fn default() -> Self {
        ProcessStatus::Disabled
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxProcessProgramLength))]
pub struct Process<
    RoleKey,
    TokenMetadataKey,
    TokenMetadataValue,
    TokenMetadataValueDiscriminator,
    MaxProcessProgramLength
> where
    RoleKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataValue: Parameter + Default + MaxEncodedLen,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue> + MaxEncodedLen,
    MaxProcessProgramLength: Get<u32>
{
    status: ProcessStatus,
    program: BoundedVec<
        BooleanExpressionSymbol<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator>,
        MaxProcessProgramLength
    >
}

impl<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator, MaxProcessProgramLength> Default
    for Process<RoleKey, TokenMetadataKey, TokenMetadataValue, TokenMetadataValueDiscriminator, MaxProcessProgramLength>
where
    RoleKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen,
    TokenMetadataValue: Parameter + Default + MaxEncodedLen,
    TokenMetadataValueDiscriminator: Parameter + Default + From<TokenMetadataValue> + MaxEncodedLen,
    MaxProcessProgramLength: Get<u32>
{
    fn default() -> Self {
        Process {
            status: ProcessStatus::Disabled,
            program: vec![BooleanExpressionSymbol::Restriction(Restriction::None)]
                .try_into()
                .unwrap()
        }
    }
}

impl<R, K, V, D, MR> PartialEq<Process<R, K, V, D, MR>> for Process<R, K, V, D, MR>
where
    R: Parameter + Default + Ord + MaxEncodedLen,
    K: Parameter + Default + Ord + MaxEncodedLen,
    V: Parameter + Default + MaxEncodedLen,
    D: Parameter + Default + From<V> + MaxEncodedLen,
    MR: Get<u32>
{
    fn eq(&self, other: &Process<R, K, V, D, MR>) -> bool {
        self.status == other.status && self.program == other.program
    }
}

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use codec::MaxEncodedLen;
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;

    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // The primary identifier for a process (i.e. it's name, and version)
        type ProcessIdentifier: Parameter + Default + MaxEncodedLen + MaybeSerializeDeserialize;
        type ProcessVersion: Parameter + AtLeast32Bit + Default + MaxEncodedLen;

        #[pallet::constant]
        type MaxProcessProgramLength: Get<u32>;

        // Origins for calling these extrinsics. For now these are expected to be root
        type CreateProcessOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type DisableProcessOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        type RoleKey: Parameter + Default + Ord + MaxEncodedLen + MaybeSerializeDeserialize;
        type TokenMetadataKey: Parameter + Default + Ord + MaxEncodedLen + MaybeSerializeDeserialize;
        type TokenMetadataValue: Parameter + Default + MaxEncodedLen + MaybeSerializeDeserialize;
        type TokenMetadataValueDiscriminator: Parameter
            + Default
            + From<Self::TokenMetadataValue>
            + MaxEncodedLen
            + MaybeSerializeDeserialize;

        // Origin for overriding weight calculation implementation
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Storage map definition
    #[pallet::storage]
    #[pallet::getter(fn process_model)]
    pub(super) type ProcessModel<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::ProcessIdentifier,
        Blake2_128Concat,
        T::ProcessVersion,
        Process<
            T::RoleKey,
            T::TokenMetadataKey,
            T::TokenMetadataValue,
            T::TokenMetadataValueDiscriminator,
            T::MaxProcessProgramLength
        >,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn version_model)]
    pub(super) type VersionModel<T: Config> =
        StorageMap<_, Blake2_128Concat, T::ProcessIdentifier, T::ProcessVersion, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub processes: Vec<(
            T::ProcessIdentifier,
            BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator
                >,
                T::MaxProcessProgramLength
            >
        )>
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { processes: Vec::new() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (process_id, program) in self.processes.iter() {
                if !Pallet::<T>::validate_program(&program) {
                    panic!("Invalid program detected in genesis!")
                }
                let version = Pallet::<T>::update_version(process_id).unwrap();
                Pallet::<T>::persist_process(process_id, &version, program).unwrap();
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // id, version, program, is_new
        ProcessCreated(
            T::ProcessIdentifier,
            T::ProcessVersion,
            BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator
                >,
                T::MaxProcessProgramLength
            >,
            bool
        ),
        //id, version
        ProcessDisabled(T::ProcessIdentifier, T::ProcessVersion)
    }

    #[pallet::error]
    pub enum Error<T> {
        // process already exists, investigate
        AlreadyExists,
        // attempting to disable non-existing process
        NonExistingProcess,
        // process is already disabled
        AlreadyDisabled,
        // process not found for this version
        InvalidVersion,
        // restriction program is invalid
        InvalidProgram
    }

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_process(program.len() as u32))]
        pub fn create_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            program: BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator
                >,
                T::MaxProcessProgramLength
            >
        ) -> DispatchResultWithPostInfo {
            T::CreateProcessOrigin::ensure_origin(origin)?;

            ensure!(Pallet::<T>::validate_program(&program), Error::<T>::InvalidProgram);

            let version: T::ProcessVersion = Pallet::<T>::update_version(&id).unwrap();
            Pallet::<T>::persist_process(&id, &version, &program)?;

            Self::deposit_event(Event::ProcessCreated(
                id,
                version.clone(),
                program,
                version == One::one()
            ));

            return Ok(().into());
        }

        #[pallet::weight(T::WeightInfo::disable_process())]
        pub fn disable_process(
            origin: OriginFor<T>,
            id: T::ProcessIdentifier,
            version: T::ProcessVersion
        ) -> DispatchResultWithPostInfo {
            T::DisableProcessOrigin::ensure_origin(origin)?;
            Pallet::<T>::validate_version_and_process(&id, &version)?;
            Pallet::<T>::set_disabled(&id, &version)?;

            Self::deposit_event(Event::ProcessDisabled(id, version));
            return Ok(().into());
        }
    }

    // helper methods
    impl<T: Config> Pallet<T> {
        pub fn validate_program(
            program: &BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator
                >,
                T::MaxProcessProgramLength
            >
        ) -> bool {
            let executed_stack_height = program.iter().try_fold(0u8, |stack_height, symbol| match symbol {
                BooleanExpressionSymbol::Op(_) => {
                    let stack_height = stack_height.checked_sub(2);
                    return stack_height.and_then(|stack_height| stack_height.checked_add(1));
                }
                BooleanExpressionSymbol::Restriction(_) => stack_height.checked_add(1)
            });
            executed_stack_height == Some(1u8)
        }

        pub fn get_next_version(id: &T::ProcessIdentifier) -> T::ProcessVersion {
            let current_version = <VersionModel<T>>::try_get(&id);
            return match current_version {
                Ok(version) => version + One::one(),
                Err(_) => One::one()
            };
        }

        pub fn update_version(id: &T::ProcessIdentifier) -> Result<T::ProcessVersion, Error<T>> {
            let version: T::ProcessVersion = Pallet::<T>::get_next_version(id);
            match version == One::one() {
                true => <VersionModel<T>>::insert(id, version.clone()),
                false => <VersionModel<T>>::mutate(id, |v| *v = version.clone())
            };

            return Ok(version);
        }

        pub fn persist_process(
            id: &T::ProcessIdentifier,
            v: &T::ProcessVersion,
            p: &BoundedVec<
                BooleanExpressionSymbol<
                    T::RoleKey,
                    T::TokenMetadataKey,
                    T::TokenMetadataValue,
                    T::TokenMetadataValueDiscriminator
                >,
                T::MaxProcessProgramLength
            >
        ) -> Result<(), Error<T>> {
            return match <ProcessModel<T>>::contains_key(&id, &v) {
                true => Err(Error::<T>::AlreadyExists),
                false => {
                    <ProcessModel<T>>::insert(
                        id,
                        v,
                        Process {
                            program: p.clone(),
                            status: ProcessStatus::Enabled
                        }
                    );
                    return Ok(());
                }
            };
        }

        pub fn set_disabled(id: &T::ProcessIdentifier, version: &T::ProcessVersion) -> Result<(), Error<T>> {
            let process = <ProcessModel<T>>::get(&id, &version);
            return match process.status == ProcessStatus::Disabled {
                true => Err(Error::<T>::AlreadyDisabled),
                false => {
                    <ProcessModel<T>>::mutate(id.clone(), version, |process| {
                        (*process).status = ProcessStatus::Disabled;
                    });
                    return Ok(());
                }
            };
        }

        pub fn validate_version_and_process(
            id: &T::ProcessIdentifier,
            version: &T::ProcessVersion
        ) -> Result<(), Error<T>> {
            ensure!(
                <ProcessModel<T>>::contains_key(&id, version.clone()),
                Error::<T>::NonExistingProcess,
            );
            ensure!(<VersionModel<T>>::contains_key(&id), Error::<T>::InvalidVersion);
            return match *version > <VersionModel<T>>::get(&id) {
                true => Err(Error::<T>::InvalidVersion),
                false => Ok(())
            };
        }
    }
}

impl<T: Config> ProcessValidator<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue> for Pallet<T> {
    type ProcessIdentifier = T::ProcessIdentifier;
    type ProcessVersion = T::ProcessVersion;

    fn validate_process(
        id: ProcessFullyQualifiedId<Self::ProcessIdentifier, Self::ProcessVersion>,
        sender: &T::AccountId,
        inputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>,
        outputs: &Vec<ProcessIO<T::AccountId, T::RoleKey, T::TokenMetadataKey, T::TokenMetadataValue>>
    ) -> bool {
        let maybe_process = <ProcessModel<T>>::try_get(id.id, id.version);

        match maybe_process {
            Ok(process) => {
                if process.status == ProcessStatus::Disabled {
                    return false;
                }

                let mut stack: Vec<bool> = Vec::with_capacity(T::MaxProcessProgramLength::get() as usize);
                for symbol in process.program {
                    match symbol {
                        BooleanExpressionSymbol::Op(op) => {
                            if let (Some(a), Some(b)) = (stack.pop(), stack.pop()) {
                                stack.push(op.eval(a, b));
                            } else {
                                return false;
                            }
                        }
                        BooleanExpressionSymbol::Restriction(r) => {
                            stack.push(validate_restriction::<
                                T::AccountId,
                                T::RoleKey,
                                T::TokenMetadataKey,
                                T::TokenMetadataValue,
                                T::TokenMetadataValueDiscriminator
                            >(r, &sender, inputs, outputs));
                        }
                    }
                }
                stack.pop().unwrap_or(false)
            }
            Err(_) => false
        }
    }
}
