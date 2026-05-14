use core::ptr;

use ohos_crypto_sys as crypto;

#[test]
fn link_smoke() {
    #[cfg(feature = "api-12")]
    unsafe {
        crypto::common::OH_Crypto_FreeDataBlob(ptr::null_mut());

        let _ = crypto::asym_key::OH_CryptoAsymKeyGenerator_Create(ptr::null(), ptr::null_mut());
        crypto::asym_key::OH_CryptoAsymKeyGenerator_Destroy(ptr::null_mut());

        let _ = crypto::digest::OH_CryptoDigest_Create(ptr::null(), ptr::null_mut());
        crypto::digest::OH_DigestCrypto_Destroy(ptr::null_mut());

        let _ = crypto::signature::OH_CryptoSign_Create(ptr::null(), ptr::null_mut());
        crypto::signature::OH_CryptoSign_Destroy(ptr::null_mut());

        let _ = crypto::sym_cipher::OH_CryptoSymCipher_Create(ptr::null(), ptr::null_mut());
        crypto::sym_cipher::OH_CryptoSymCipher_Destroy(ptr::null_mut());

        let _ = crypto::sym_key::OH_CryptoSymKeyGenerator_Create(ptr::null(), ptr::null_mut());
        crypto::sym_key::OH_CryptoSymKeyGenerator_Destroy(ptr::null_mut());
    }

    #[cfg(feature = "api-20")]
    unsafe {
        let _ = crypto::asym_cipher::OH_CryptoAsymCipher_Create(ptr::null(), ptr::null_mut());
        crypto::asym_cipher::OH_CryptoAsymCipher_Destroy(ptr::null_mut());

        let _ = crypto::kdf::OH_CryptoKdf_Create(ptr::null(), ptr::null_mut());
        crypto::kdf::OH_CryptoKdf_Destroy(ptr::null_mut());

        let _ = crypto::key_agreement::OH_CryptoKeyAgreement_Create(ptr::null(), ptr::null_mut());
        crypto::key_agreement::OH_CryptoKeyAgreement_Destroy(ptr::null_mut());

        let _ = crypto::mac::OH_CryptoMac_Create(ptr::null(), ptr::null_mut());
        crypto::mac::OH_CryptoMac_Destroy(ptr::null_mut());

        let _ = crypto::rand::OH_CryptoRand_Create(ptr::null_mut());
        crypto::rand::OH_CryptoRand_Destroy(ptr::null_mut());
    }
}
