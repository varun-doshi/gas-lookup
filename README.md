# Gas-Lookup

A cli tool to calculate total gas spent by an address within a given time frame.

![Static Badge](https://img.shields.io/badge/v-0.1.3-blue)
[![crates](https://img.shields.io/badge/crates.io-000000?style=for-the-badge&logo=rust&logoColor=white)](https://crates.io/crates/gas-lookup)
[![github](https://img.shields.io/badge/github-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/varun-doshi/gas-lookup)
[![twitter](https://img.shields.io/badge/twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/Varunx10)

##

A CLI tool to calculate the total amount of Ether spent on gas on transacting on the Ethereum blockchain withing a given date frame.

## Installation

Install gas-lookup with cargo

```bash
  cargo install gas-lookup
```

## Usage

Once the install is completed, the cli command will be available globally on your system.
To run the application:

```bash
gas-lookup <ethereum_address> <start date> <end date>
```

## Parameters passed are:

- `ethereum address`: The Ethereum mainnet address whose gas spent needs to be calculated
- `start date`: The start date to calculate gas from. Format - `dd/mm/yyy`
- `end data`: The date till which gas needs to be calculated. Format - `dd/mm/yyy`

For help,

```bash
  gas-lookup --help
```

## Screenshots

![gas-lookup](https://i.postimg.cc/BbV9brq2/Screenshot-2023-12-25-124056.jpg)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
