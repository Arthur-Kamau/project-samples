# Parking Application Rust Smart Contract 

## Getting started
## About 
Parking App Rust Smart Contract creates a platform where drivers around busy cities are able to find suitable parking spaces for their vehicles hence saving time wasted searchig around for spaces physically

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
 ## Driver Struct
    Contains the property of a driver instance.That holds on details of the driver
 ## Parkingslot Struct
    Contains the Property of the locations. That will hold driver details that will be given by the driver.
 
 
 ## Implementation of my App 
  Inside involves the following state operations :
  
  1. Function to allocate an empty slot to driver
  2. Counting the  number of parking slot
  3. Function to add driver
  4. Function to add a parking slot
  5. Function to remove a parking slot
 
## Tests
For the Smart contract using the Command :

    Cargo test
    or
    Cargo test -- --nocapture
Test to confirm of sucess of my Contract
## Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
## Working with Smart Contract

###  allocating an empty slot
    near call dev-1655165689664-15852609649098 add_movie '{"movie_name":"Nightmare","author":"Smith","stream_platform":"Netflix","rated":"GE" }' --account-id jerryj.testnet



**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
