pub mod integration_testing {
    use frame_support::derive_impl;
    use sp_runtime::BuildStorage;
    use frame_support::{assert_ok};
    use frame_system::*;
    use sp_runtime::{AccountId32};
    use crate::*;
    use super::*;

    // Define the type of block for the runtime.
    type Block = frame_system::mocking::MockBlock<Runtime>;

    // Build genesis storage according to the runtime's configuration.
    pub fn new_test_ext() -> sp_io::TestExternalities {
        frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap().into()
    }

    #[test]
    fn testing_runtime() {
        new_test_ext().execute_with(|| {
            // Block 0: Check if initialized correctly
            assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 0);

            let result = pallet_template::Pallet::<Runtime>::dummy_call(RuntimeOrigin::none());

            // Transition to block 1.
            frame_system::Pallet::<Runtime>::set_block_number(1);

            // Check if block number is now 1.
            assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 1);

        });
    }
}