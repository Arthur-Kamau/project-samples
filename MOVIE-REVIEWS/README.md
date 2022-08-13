# Movie Reviews Rust Smart Contract 

## Getting started
## About 
Movie Reviews Rust Smart Contract creates a platform where we have 
an awareness of a Movie to the App with details such as Movie name, author , Stream Platform and Rated .Then for the User he/she gets to give Review of the movie and even give feedback on their best Qoutes they heard on the film.

# Tools for development
## Rust
If you’re a Windows Subsystem for Linux user run the following in your terminal, then follow the on-screen instructions to install Rust.
To get started with this template:
 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then checking version :

    rustc --version
## Node and Npm

In a web browser, navigate to https://nodejs.org/en/download/. The Node.js installer includes the NPM package manager.
Then for verification of installation run , Open a command prompt (or PowerShell), and enter the following:

    node -v
    npm -v

## Near Cli
Installation
Make sure you have a current version of npm and NodeJS installed.

### **Mac and Linux**
1.Install npm and node using a package manager like nvm as sometimes there are issues using Ledger due to how OS X handles node packages related to USB devices. [click here]
2.Ensure you have installed Node version 12 or above.
Install near-cli globally by running:

    npm install -g near-cli

### **Windows**
For Windows users, we recommend using Windows Subsystem for Linux (WSL).

1.Install WSL [click here]
2.Install npm [click here]
3.Install Node.js [ click here ]
4.Change npm default directory [ click here ]
This is to avoid any permission issues with WSL
5.Open WSL and install near-cli globally by running:

    npm install -g near-cli

# Summary of the Code 
 ## Dependancies 
    Initialization for dependancies with Borsh being the recommended serialization method for near smart contract development.
 ## Movie Struct
    Contains the property of a Movie instance.That holds on details of the Movie
 ## Review Struct
    Contains the Property of the Movie Reviews. That will hold Review details that will be given by the Reviewer.
 ## App  Struct 
    I have a moviestore that hold my values  of the Movie in a Hashmap

    I have a reviewstore that hold my values  of the Review in a Hashmap
 ## Default Implemantation 
    Initializing the initial State of my moviestore and reviewstore.
 ## Implementation of my App 
  Inside involves the following state operations :
  
  1. Function to add Movie 
  2. Counting Number of Movies in my moviestore
  3. Function to get a Movie from my moviestore
  4. Function to remove a Movie
  5. Function to add Movie Reviews 
  6. Function to Count Number of Movie Reviews
  7. Function to get Movie Reviews from reviewstore
  8. Function to Remove a Movie Review.

## Tests
For the Smart contract using the Command :

    Cargo test
    or
    Cargo test -- --nocapture
Test to confirm of sucess of my Contract
## Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
## Working with Smart Contract

###  Adding a Movie 
    near call dev-1655165689664-15852609649098 add_movie '{"movie_name":"Nightmare","author":"Smith","stream_platform":"Netflix","rated":"GE" }' --account-id jerryj.testnet

###  Counting my Movies
    near call dev-1655165689664-15852609649098 movie_count  --account-id jerryj.testnet
### Getting a Movie
    near call dev-1655165689664-15852609649098 get_movie '{"movie_name":"Nightmare" }' --account-id jerryj.testnet
### Deleting a Movie 
    near call dev-1655165689664-15852609649098 remove_movie '{"movie_name":"Nightmare" }' --account-id jerryj.testnet
### Adding a Movie Review
    near call dev-1655165689664-15852609649098 add_review '{"movie_name":"Nightmare","description":"Thriller based movie","reviewer":"Jerry Joseph","best_quote":"Whatever you do do not fall asleep" }' --account-id jerryj.testnet
### Getting a Movie Review
    near call dev-1655165689664-15852609649098 get_review '{"movie_name":"Nightmare" }' --account-id jerryj.testnet
### Counting my Movie Reviews
    near call dev-1655165689664-15852609649098 review_count  --account-id jerryj.testnet
### Deleting a Movie Review
    near call dev-1655165689664-15852609649098 remove_review '{"movie_name":"Nightmare" }' --account-id jerryj.testnet



**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
