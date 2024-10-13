use ic_cdk_macros::{init, update, query};
use candid::{CandidType, Deserialize};
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

// Submit a KYC request
#[update]
fn submit_kyc(user_id: String, document: String) -> bool {
    unsafe {
        let kyc_requests = KYC_REQUESTS.as_mut().unwrap();
        if kyc_requests.contains_key(&user_id) {
            return false; // KYC request already exists
        }

        let kyc_request = KYCRequest {
            user_id: user_id.clone(),
            document,
            status: "Pending".to_string(),
        };

        kyc_requests.insert(user_id, kyc_request);
        true
    }
}

// Get the status of a KYC request
#[query]
fn get_kyc_status(user_id: String) -> Option<KYCRequest> {
    unsafe {
        let kyc_requests = KYC_REQUESTS.as_ref().unwrap();
        kyc_requests.get(&user_id).cloned()
    }
}

// Approve a KYC request
#[update]
fn approve_kyc(user_id: String) -> bool {
    unsafe {
        let kyc_requests = KYC_REQUESTS.as_mut().unwrap();
        if let Some(kyc_request) = kyc_requests.get_mut(&user_id) {
            kyc_request.status = "Approved".to_string();
            return true;
        }
        false
    }
}

// Reject a KYC request
#[update]
fn reject_kyc(user_id: String) -> bool {
    unsafe {
        let kyc_requests = KYC_REQUESTS.as_mut().unwrap();
        if let Some(kyc_request) = kyc_requests.get_mut(&user_id) {
            kyc_request.status = "Rejected".to_string();
            return true;
        }
        false
    }
}
