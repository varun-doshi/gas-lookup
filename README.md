# Gas-Lookup

A cli tool to calculate total gas spent by an address within a given time frame.

![Static Badge](https://img.shields.io/badge/v-0.1.7-blue)
[![crates](https://img.shields.io/badge/crates.io-000000?style=for-the-badge&logo=rust&logoColor=white)](https://crates.io/crates/gas-lookup)
[![github](https://img.shields.io/badge/github-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/varun-doshi/gas-lookup)
[![twitter](https://img.shields.io/badge/twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/Varunx10)

##

A CLI tool to calculate the total amount of Ether spent on gas on transacting on a specific blockchain withing a given date frame.

## Installation

Install gas-lookup with cargo

```bash
  cargo install gas-lookup
```

## Usage

Once the install is completed, the cli command will be available globally on your system.
To run the application:

```bash
gas-lookup <address> <start_date> <end_date> [chain]
```

## Parameters passed are:

- `address`: The address whose gas spent needs to be calculated
- `start date`: The start date to calculate gas from. Format - `dd/mm/yyy`
- `end date`: The date till which gas needs to be calculated. Format - `dd/mm/yyy`
- [OPTIONAL]`chain`: Specify the chain to claculate gas on. Defaults to Ethereum Mainnet
  Available options:
  - `eth`: For Ethereum Mainnet
  - `pol`: For Polygon Mainnet
  - `opt`: For Optimism Mainnet

For help,

```bash
  gas-lookup --help
```

## Screenshots

![gas-lookup](https://i.postimg.cc/BbV9brq2/Screenshot-2023-12-25-124056.jpg)

## Updates

### v0.1.7

Updated code to correct bug which caused the binary file to crash for certain cases.

### v0.1.6

Support for Polygon Mainnet and Optimism Mainnet has been added. Use the `chain` flag to specific desired chain.

```bash
gas-lookup <address_OR_ENS_name> <start_date> <end_date> [chain]
```

### v0.1.5

Support for ENS names have been added. Search with either ETH address or ENS name

```bash
gas-lookup <ETH_address OR ENS_name> <start_date> <end_date>
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
