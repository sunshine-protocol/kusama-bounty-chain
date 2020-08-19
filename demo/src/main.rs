use kb_client::{
    bounty::BountyClient,
    client::{
        Client as _,
        Result,
    },
    mock::AccountKeyring,
    Client,
    GithubIssue,
};
use sunshine_crypto::{
    keychain::TypedPair,
    secrecy::SecretString,
};

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();
    let alice_root = dirs::config_dir().unwrap().join("demo-alice");
    let mut alice_client =
        Client::new(&alice_root, "ws://127.0.0.1:9944").await?;
    alice_client
        .set_key(
            TypedPair::from_suri(&AccountKeyring::Alice.to_seed()).unwrap(),
            &SecretString::new("password".to_string()),
            true,
        )
        .await?;
    alice_client
        .unlock(&SecretString::new("password".to_string()))
        .await?;
    let bob_root = dirs::config_dir().unwrap().join("demo-bob");
    let mut bob_client = Client::new(&bob_root, "ws://127.0.0.1:9944").await?;
    bob_client
        .set_key(
            TypedPair::from_suri(&AccountKeyring::Bob.to_seed()).unwrap(),
            &SecretString::new("password".to_string()),
            true,
        )
        .await?;
    bob_client
        .unlock(&SecretString::new("password".to_string()))
        .await?;
    let bounty = GithubIssue {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 160,
    };
    // (1) Alice posts a bounty!
    let _ = alice_client.post_bounty(bounty, 2000).await?;
    let submission = GithubIssue {
        repo_owner: "sunshine-protocol".to_string(),
        repo_name: "sunshine-bounty".to_string(),
        issue_number: 161,
    };
    // (2) Bob submits for Alice's bounty!
    let _ = bob_client.submit_for_bounty(1, submission, 1000).await?;
    // (3) Alice approves Bob's submission
    let _ = alice_client.approve_bounty_submission(1).await?;
    Ok(())
}
