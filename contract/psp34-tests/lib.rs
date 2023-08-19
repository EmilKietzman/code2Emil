#[cfg(test)]
mod tests {
    
    use crate::psp34_contract::PSP34 as _;
    use aleph_client::keypair_from_string;
    use aleph_client::Connection;
    use aleph_client::SignedConnection;
    use anyhow::Result;
    use ink_wrapper_types::util::ToAccountId as _;
    use rand::RngCore as _;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        // Connect to the node launched earlier.
        let conn = Connection::new("ws://localhost:9944").await;
        let conn = SignedConnection::from_connection(conn, keypair_from_string("//Alice"));
        let bob = keypair_from_string("//Bob");

        // We're using a random salt here so that each test run is independent.
        let mut salt = vec![0; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        let total_supply = 1000;
        // Constructors take a connection, the salt, and any arguments
        // the actual constructor requires afterwards.
        let contract = psp34_contract::Instance::new(&conn, salt, total_supply).await?;

        // A mutating method takes a signed connection and any arguments afterwards.
        contract
            .transfer(&conn, bob.account_id().to_account_id(), 100, vec![])
            .await?;

        // A reader method takes a connection (may be unsigned) and any arguments afterwards.
        let balance = contract
            .balance_of(&conn, bob.account_id().to_account_id())
            .await??;
        assert_eq!(balance, 100);

        Ok(())
    }
}
