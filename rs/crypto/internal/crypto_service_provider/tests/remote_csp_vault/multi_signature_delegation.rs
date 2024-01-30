use ic_crypto_internal_csp_proptest_utils::{
    arb_algorithm_id, arb_csp_multi_signature_error, arb_csp_multi_signature_keygen_error,
    arb_csp_pop, arb_csp_public_key, arb_csp_signature, arb_key_id,
};
use ic_crypto_temp_crypto_vault::RemoteVaultEnvironment;
use ic_crypto_test_utils_local_csp_vault::MockLocalCspVault;
use proptest::collection::vec;
use proptest::prelude::any;
use proptest::result::maybe_err;
use proptest::{prop_assert_eq, proptest};
use std::sync::Arc;

mod common;
use common::proptest_config_for_delegation;

proptest! {
    #![proptest_config(proptest_config_for_delegation())]
    #[test]
    fn should_delegate_for_multi_sign(
        algorithm_id in arb_algorithm_id(),
        key_id in arb_key_id(),
        message in vec(any::<u8>(), 0..1024),
        expected_result in maybe_err(arb_csp_signature(), arb_csp_multi_signature_error())
    ) {
        let expected_message = message.clone();
        let mut local_vault = MockLocalCspVault::new();
        local_vault
            .expect_multi_sign()
            .times(1)
            .withf(move |algorithm_id_, message_, key_id_| {
                *algorithm_id_ == algorithm_id && message_ == &expected_message && *key_id_ == key_id
            })
            .return_const(expected_result.clone());
        let env = RemoteVaultEnvironment::start_server_with_local_csp_vault(Arc::new(local_vault));
        let remote_vault = env.new_vault_client();

        let result = remote_vault.multi_sign(algorithm_id, message, key_id);

        prop_assert_eq!(result, expected_result);
    }
}

proptest! {
    #![proptest_config(proptest_config_for_delegation())]
    #[test]
    fn should_delegate_for_gen_committee_signing_key_pair(
        expected_result in maybe_err((arb_csp_public_key(), arb_csp_pop()), arb_csp_multi_signature_keygen_error())
    ) {
        let mut local_vault = MockLocalCspVault::new();
        local_vault
            .expect_gen_committee_signing_key_pair()
            .times(1)
            .return_const(expected_result.clone());
        let env = RemoteVaultEnvironment::start_server_with_local_csp_vault(Arc::new(local_vault));
        let remote_vault = env.new_vault_client();

        let result = remote_vault.gen_committee_signing_key_pair();

        prop_assert_eq!(result, expected_result);
    }
}
