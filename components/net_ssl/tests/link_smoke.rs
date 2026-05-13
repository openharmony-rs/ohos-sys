#![cfg(feature = "api-11")]

use ohos_net_ssl_sys as net_ssl;

#[test]
fn link_smoke() {
    unsafe {
        let mut cert_bytes = b"-----BEGIN CERTIFICATE-----\n-----END CERTIFICATE-----\n".to_vec();
        let cert = net_ssl::net_ssl_c_type::NetStack_CertBlob {
            type_: net_ssl::net_ssl_c_type::NetStack_CertType::NETSTACK_CERT_TYPE_PEM,
            size: cert_bytes.len() as u32,
            data: cert_bytes.as_mut_ptr(),
        };
        let _ = net_ssl::net_ssl_c::OH_NetStack_CertVerification(&cert, core::ptr::null());
    }
}
