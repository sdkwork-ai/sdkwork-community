//! Cross-service integration for the community service (membership package
//! registration and order payment verification).

pub mod commerce;
mod commerce_http;

pub use commerce::{
    is_paid_order_status, CommerceIntegration, CommerceIntegrationConfig, CommerceScope,
    CommerceTransportMode, MembershipPackagePublishFuture, MembershipPackagePublisher,
    MembershipPackageRegistration, OrderPaymentVerification, OrderPaymentVerifier,
    OrderPaymentVerifyFuture, RegisteredMembershipPackage,
};
