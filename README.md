# AWS Lambda functionality for Rustybeer

This project is used to create AWS Lambda functionalities that use
[Rustybeer](https://github.com/drodil/rustybeer) library for calculations.

## Setup

Install the following dependencies:

* [AWS CLI](https://aws.amazon.com/cli/)
* [Rust](https://rustup.rs/)
* [NodeJS](https://nodejs.org/en/download/)
* [Docker](https://www.docker.com/)

Once all that is done, install the following packages:

```bash
rustup target add x86_64-unknown-linux-musl
npm install -g serverless
npm install --save-dev serverless-rust
```

Additionally you need cross compiler for x86_64_musl. On mac this can be
installed using homebrew:

```bash
brew tap SergioBenitez/osxctd
brew install FiloSottile/musl-cross/musl-cross
```

On Ubuntu:

```bash
apt-get install musl-tools
```

And setup your AWS CLI for deployments:

```bash
aws configure
```

You can get your tokens from [AWS
IAM](https://console.aws.amazon.com/iam/home?region=eu-west-1#/security_credentials)

## Running locally

You can run lambda functions locally with serverless:

```bash
serverless invoke local -f FUNCTION -d JSON_PAYLOAD
```

## Deploying

When you have your AWS CLI set up you can easily deploy the endpoints to
production with serverless

```bash
serverless deploy
```

