# Kusama Bounty Chain + Bot

This substrate chain implementation supports posting and crowdfunding bounties. *It uses [sunshine-keybase](https://github.com/sunshine-protocol/sunshine-keybase) for identity and device authentication.*

`Post` bounty user flow:
1. Open an issue on github with the bounty's requirements
2. Use the mobile interface to post a new bounty. 

`Submit` entry user flow:
1. Open a separate issue to track the submission's review.
2. Use the mobile interface to post a new bounty submission. 

Upon transaction inclusion, a bot watching the chain posts in the github issues with success messages containing relevant on-chain data (i.e. amount backing the bounty, amount requested by the submission, etc). 

Submission approval and outside contributions are handled in the mobile interface, and the bot emits relevant messages in the Bounty/Submission github issues to report any changes to on-chain state.

### Did this code exist before the submission period?

The code was completed during the submission period. Neither the bot code nor the bounty module worked with the Rust client until this PR https://github.com/sunshine-protocol/sunshine-bounty/pull/144 was merged August 9th. Since then, a few more PRs were required to set up the testnet, achieve compatibility with the interface, and debug the github bot. The project was completed the final day of submissions.

## Demo Instructions

### Live Testnet

The live testnet is deployed from [sunshine](https://github.com/sunshine-protocol/sunshine). To demo the Android bounty interface with the live testnet, download the APK [here](https://github.com/sunshine-protocol/sunshine-bounty-ui/releases). **[@shekohex](https://github.com/shekohex)** posted a [video demo](https://www.youtube.com/watch?v=bQSYjOT1R04&feature=youtu.be) 🚀

You can
* post a bounty
* contribute to a bounty crowdfund
* submit for a bounty
* approve a bounty

To post a bounty, a github issue must first be made with the bounty details in a comment (any format). The issue URL is required by the Android interface to form the bounty post command (along with the amount). Similarly, submissions for bounties require an issue URL (different from the bounty issue) that contains submission information in a comment (any format).

The [testnet bot](https://github.com/sunshine-bors) watches the testnet chain and dutifully reports any changes to chain state in relevant github issues!

### Test Locally

1. **Run the node** in dev mode.
```sh
cd node
cargo build --release
cd ..
./target/release/kb-test-node --dev
```

2. **Run the local [demo bot](https://github.com/ksm-bounty-bot)**. 

Open a new window. First, create a new file in the root called `config.sh`. In this file, paste the text from [this pastebin](https://pastebin.com/hLuEK5Ty) which follows the format:
```sh
export GITHUB_TOKEN="$SOME_NUMBER"
```
Initialize the token as an environment variable for the bot
```sh
source config.sh # initialize the GITHUB_TOKEN env var for the bot
cd bot
cargo run # Ctrl + C to kill
```

> Before there was a file `config.sh` placed in the root of this repo, but Github automatically revokes any publicly exposed token.

3. **Run the demo**.

Open a new window. Ensure the node and bot are running in the other windows. Then run
```sh
cd demo
cargo run --release
```

Check the demo hardcoded [bounty issue](https://github.com/sunshine-protocol/sunshine-bounty/issues/160) and [submission issue](https://github.com/sunshine-protocol/sunshine-bounty/issues/161) to see the results!

### notable node commands 

`Ctrl + C` to kill the node.

To purge the chain from the root of this directory
```sh
./target/release/kb-test-node purge-chain --dev
```