use super::{keypair::KeyPair, service::MultisigService};
use crate::{
    proto::{Algorithm, KeygenRequest},
    TofndResult,
};
use anyhow::anyhow;

impl MultisigService {
    pub(super) async fn handle_keygen(&self, request: &KeygenRequest) -> TofndResult<Vec<u8>> {
        let algorithm = Algorithm::try_from(request.algorithm)
            .map_err(|_| anyhow!("Invalid algorithm: {}", request.algorithm))?;
        let secret_recovery_key = self.kv_manager.seed().await?;

        println!("secret_recovery_key: {:?}", secret_recovery_key);

        Ok(
            KeyPair::new(&secret_recovery_key, request.key_uid.as_bytes(), algorithm)?
                .encoded_verifying_key(),
        )
    }
}
