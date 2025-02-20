use crate::server::api::{v2, v3};
use raiko_core::interfaces::ProofRequest;
use raiko_lib::proof_type::ProofType;
use raiko_reqpool::Status;
use raiko_tasks::TaskStatus;

pub fn to_v2_status(proof_type: ProofType, result: Result<Status, String>) -> v2::Status {
    match result {
        Ok(status) => v2::Status::Ok {
            proof_type,
            data: {
                match status {
                    Status::Registered => v2::ProofResponse::Status {
                        status: TaskStatus::Registered,
                    },
                    Status::WorkInProgress => v2::ProofResponse::Status {
                        status: TaskStatus::WorkInProgress,
                    },
                    Status::Cancelled => v2::ProofResponse::Status {
                        status: TaskStatus::Cancelled,
                    },
                    Status::Failed { error } => v2::ProofResponse::Status {
                        status: TaskStatus::AnyhowError(error),
                    },
                    Status::Success { proof } => v2::ProofResponse::Proof { proof },
                }
            },
        },
        Err(e) => v2::Status::Error {
            error: "task_failed".to_string(),
            message: e,
        },
    }
}

pub fn to_v2_cancel_status(result: Result<Status, String>) -> v2::CancelStatus {
    match result {
        Ok(status) => match status {
            Status::Success { .. } | Status::Cancelled | Status::Failed { .. } => {
                v2::CancelStatus::Ok
            }
            _ => v2::CancelStatus::Error {
                error: "cancel_failed".to_string(),
                message: format!("cancallation response unexpected status {}", status),
            },
        },
        Err(e) => v2::CancelStatus::Error {
            error: "cancel_failed".to_string(),
            message: e,
        },
    }
}

// TODO: remove the staled interface
pub fn to_v3_status(proof_type: ProofType, result: Result<Status, String>) -> v3::Status {
    to_v2_status(proof_type, result)
}

pub fn to_v3_cancel_status(result: Result<Status, String>) -> v3::CancelStatus {
    to_v2_cancel_status(result)
}

// /// Macro to extract an ID from prover args based on proof type and field name
// macro_rules! extract_image_id {
//     ($prover_args:expr, $proof_type:expr, $field:expr) => {
//         $prover_args
//             .get($proof_type)
//             .expect(&assertion_message)
//             .get($field)
//             .expect(&assertion_message)
//             .as_str()
//             .expect(&assertion_message)
//             .to_string()
//     };
// }

// /// Extract the image ID from the proof request, or return the default image ID if not set.
// pub fn extract_image_id_from_request(request: &ProofRequest) -> String {
//     let assertion_message = format!("failed to get image id of request {:?}", request);
//     let prover_args = &request.prover_args;
//     match request.proof_type {
//         ProofType::Native => String::new(),
//         ProofType::Sgx => extract_image_id!(prover_args, "sgx", "instance_id"),
//         ProofType::Sp1 =>
//         ProofType::Risc0 => extract_image_id!(prover_args, "risc0", "image_id"),
//     }
// }
