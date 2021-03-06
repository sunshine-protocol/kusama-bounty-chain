use ipld_block_builder::ReadonlyCache;
use kb_client::{
    bounty::{
        Bounty,
        BountyEventsDecoder,
        BountyPaymentExecutedEvent,
        BountyPostedEvent,
        BountyRaiseContributionEvent,
        BountySubmissionPostedEvent,
    },
    client::{
        Client as _,
        Result,
    },
    Client,
    GithubIssue,
    Runtime,
};
use std::{
    marker::PhantomData,
    sync::Arc,
};
use substrate_subxt as subxt;
use sunshine_bounty_gbot::GBot;
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let github = GBot::new()?;
    let root = dirs::config_dir().unwrap().join("sunshine-bounty-bot");
    let client = Arc::new(Client::new(&root, "ws://127.0.0.1:9944").await?);

    let post = Subscription::<_, BountyPostedEvent<Runtime>>::subscribe(
        client.chain_client(),
    )
    .await?;
    let contrib =
        Subscription::<_, BountyRaiseContributionEvent<Runtime>>::subscribe(
            client.chain_client(),
        )
        .await?;
    let submit =
        Subscription::<_, BountySubmissionPostedEvent<Runtime>>::subscribe(
            client.chain_client(),
        )
        .await?;
    let approval =
        Subscription::<_, BountyPaymentExecutedEvent<Runtime>>::subscribe(
            client.chain_client(),
        )
        .await?;

    let post =
        task::spawn(process_subscription(client.clone(), github.clone(), post));
    let contrib = task::spawn(process_subscription(
        client.clone(),
        github.clone(),
        contrib,
    ));
    let submit = task::spawn(process_subscription(
        client.clone(),
        github.clone(),
        submit,
    ));
    let approval = task::spawn(process_subscription(client, github, approval));

    post.await?;
    contrib.await?;
    submit.await?;
    approval.await?;

    Ok(())
}

pub struct Subscription<R: subxt::Runtime, E: subxt::Event<R>> {
    _marker: PhantomData<E>,
    subscription: subxt::EventSubscription<R>,
}

impl<R: subxt::Runtime + Bounty, E: subxt::Event<R>> Subscription<R, E> {
    async fn subscribe(client: &subxt::Client<R>) -> Result<Self> {
        let sub = client.subscribe_events().await?;
        let mut decoder =
            subxt::EventsDecoder::<R>::new(client.metadata().clone());
        decoder.with_bounty();
        let mut subscription = subxt::EventSubscription::<R>::new(sub, decoder);
        subscription.filter_event::<E>();
        Ok(Self {
            _marker: PhantomData,
            subscription,
        })
    }

    async fn next(&mut self) -> Option<Result<E>> {
        match self.subscription.next().await {
            Some(Ok(raw)) => {
                Some(E::decode(&mut &raw.data[..]).map_err(Into::into))
            }
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

async fn process_subscription<E: subxt::Event<Runtime> + Into<Event>>(
    client: Arc<Client>,
    github: GBot,
    mut subscription: Subscription<Runtime, E>,
) {
    while let Some(res) = subscription.next().await {
        if let Err(err) =
            process_event(&client, &github, res.map(Into::into)).await
        {
            log::error!("{:?}", err);
        }
    }
}

pub enum Event {
    BountyPosted(BountyPostedEvent<Runtime>),
    RaiseContribution(BountyRaiseContributionEvent<Runtime>),
    SubmissionPosted(BountySubmissionPostedEvent<Runtime>),
    PaymentExecuted(BountyPaymentExecutedEvent<Runtime>),
}

impl From<BountyPostedEvent<Runtime>> for Event {
    fn from(ev: BountyPostedEvent<Runtime>) -> Self {
        Self::BountyPosted(ev)
    }
}

impl From<BountyRaiseContributionEvent<Runtime>> for Event {
    fn from(ev: BountyRaiseContributionEvent<Runtime>) -> Self {
        Self::RaiseContribution(ev)
    }
}

impl From<BountySubmissionPostedEvent<Runtime>> for Event {
    fn from(ev: BountySubmissionPostedEvent<Runtime>) -> Self {
        Self::SubmissionPosted(ev)
    }
}

impl From<BountyPaymentExecutedEvent<Runtime>> for Event {
    fn from(ev: BountyPaymentExecutedEvent<Runtime>) -> Self {
        Self::PaymentExecuted(ev)
    }
}

async fn process_event(
    client: &Client,
    github: &GBot,
    event: Result<Event>,
) -> Result<()> {
    match event? {
        Event::BountyPosted(event) => {
            // fetch structured data from client
            let event_cid = event.description.to_cid()?;
            let bounty_body: GithubIssue =
                client.offchain_client().get(&event_cid).await?;
            // new issue comment
            github
                .new_bounty_issue(
                    event.amount,
                    event.id,
                    bounty_body.repo_owner,
                    bounty_body.repo_name,
                    bounty_body.issue_number,
                )
                .await?;
        }
        Event::RaiseContribution(event) => {
            // fetch structured data from client
            let event_cid = event.bounty_ref.to_cid()?;
            let bounty_body: GithubIssue =
                client.offchain_client().get(&event_cid).await?;
            // update existing bounty comment
            github
                .update_bounty_issue(
                    event.total,
                    event.bounty_id,
                    bounty_body.repo_owner,
                    bounty_body.repo_name,
                    bounty_body.issue_number,
                )
                .await?;
        }
        Event::SubmissionPosted(event) => {
            // fetch structured data from client
            let bounty_event_cid = event.bounty_ref.to_cid()?;
            let submission_event_cid = event.submission_ref.to_cid()?;
            let bounty_body: GithubIssue =
                client.offchain_client().get(&bounty_event_cid).await?;
            let submission_body: GithubIssue =
                client.offchain_client().get(&submission_event_cid).await?;
            // new issue comment
            github
                .new_submission_issue(
                    event.amount,
                    event.bounty_id,
                    event.id,
                    bounty_body.repo_owner,
                    bounty_body.repo_name,
                    bounty_body.issue_number,
                    submission_body.repo_owner,
                    submission_body.repo_name,
                    submission_body.issue_number,
                )
                .await?;
        }
        Event::PaymentExecuted(event) => {
            // fetch structured data from client
            let bounty_event_cid = event.bounty_ref.to_cid()?;
            let submission_event_cid = event.submission_ref.to_cid()?;
            let bounty_body: GithubIssue =
                client.offchain_client().get(&bounty_event_cid).await?;
            let submission_body: GithubIssue =
                client.offchain_client().get(&submission_event_cid).await?;
            // update existing submission comment
            github
                .approve_submission_issue(
                    event.amount,
                    event.bounty_id,
                    event.submission_id,
                    bounty_body.repo_owner.clone(),
                    bounty_body.repo_name.clone(),
                    bounty_body.issue_number,
                    submission_body.repo_owner,
                    submission_body.repo_name,
                    submission_body.issue_number,
                )
                .await?;
            // update existing bounty comment
            github
                .update_bounty_issue(
                    event.new_total,
                    event.bounty_id,
                    bounty_body.repo_owner,
                    bounty_body.repo_name,
                    bounty_body.issue_number,
                )
                .await?;
        }
    }
    Ok(())
}
