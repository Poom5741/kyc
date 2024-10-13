use ic_cdk::api::management_canister::ecdsa::{SignWithEcdsaArgument, SignWithEcdsaResponse};
use ic_cdk_macros::{init, update, query};
use candid::{CandidType, Deserialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Define the KYC request structure
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct KYCRequest {
    user_id: String,
    document: String,
    status: String,
}

// Define the type for KYC request storage
type KYCStore = HashMap<String, KYCRequest>;

// The KYC requests map will be stored in stable memory
static mut KYC_REQUESTS: Option<KYCStore> = None;

// Initialize the KYC request store
#[init]
fn init() {
    unsafe {
        KYC_REQUESTS = Some(HashMap::new());
    }
}

// Helper function to access KYC_REQUESTS safely
fn get_kyc_requests() -> &'static mut KYCStore {
    unsafe {
        KYC_REQUESTS.get_or_insert_with(HashMap::new)
    }
}

// Submit a KYC request with cryptographic signing
#[update]
async fn submit_kyc(user_id: String, document: String) -> bool {
    let kyc_requests = get_kyc_requests();

    if kyc_requests.contains_key(&user_id) {
        return false; // KYC request already exists
    }

    // Hash the document data
    let mut hasher = Sha256::new();
    hasher.update(&document);
    let document_hash = hasher.finalize();

    // Prepare the argument for ECDSA signing
    let ecdsa_argument = SignWithEcdsaArgument {
        message_hash: document_hash.to_vec(),
        derivation_path: vec![], // You can define this according to your use case
        key_id: ic_cdk::api::management_canister::ecdsa::EcdsaKeyId {
            name: "dfx_test_key".to_string(),
            curve: ic_cdk::api::management_canister::ecdsa::EcdsaCurve::Secp256k1,
        }, // Use the correct key id for your environment
    };

    // Call sign_with_ecdsa (this needs to be async)
    let signature_response: (SignWithEcdsaResponse,) = 
        ic_cdk::api::management_canister::ecdsa::sign_with_ecdsa(ecdsa_argument).await.unwrap();

    let signature = signature_response.0.signature;

    // Store the signed KYC request
    let kyc_request = KYCRequest {
        user_id: user_id.clone(),
        document: format!("Signed document: {:?}", signature),
        status: "Pending".to_string(),
    };

    kyc_requests.insert(user_id, kyc_request);
    true
}

// Get the status of a KYC request
#[query]
fn get_kyc_status(user_id: String) -> Option<KYCRequest> {
    let kyc_requests = get_kyc_requests();
    kyc_requests.get(&user_id).cloned()
}

// Approve a KYC request
#[update]
fn approve_kyc(user_id: String) -> bool {
    let kyc_requests = get_kyc_requests();
    if let Some(kyc_request) = kyc_requests.get_mut(&user_id) {
        kyc_request.status = "Approved".to_string();
        return true;
    }
    false
}

// Reject a KYC request
#[update]
fn reject_kyc(user_id: String) -> bool {
    let kyc_requests = get_kyc_requests();
    if let Some(kyc_request) = kyc_requests.get_mut(&user_id) {
        kyc_request.status = "Rejected".to_string();
        return true;
    }
    false
}
