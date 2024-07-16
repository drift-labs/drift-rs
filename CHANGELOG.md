# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/drift-labs/drift-rs/releases/tag/v0.1.0) - 2024-07-16

### Added
- add support for depth parameter in orderbook query ([#6](https://github.com/drift-labs/drift-rs/pull/6))
- feat/ci ([#1](https://github.com/drift-labs/drift-rs/pull/1))

### Fixed
- fix issue with awaits
- fix liq price w spot collateral ([#18](https://github.com/drift-labs/drift-rs/pull/18))

### Other
- Hotfix/pull oracles ([#42](https://github.com/drift-labs/drift-rs/pull/42))
- update drift program for pyth pull fork
- update drift program for pyth pull fork
- Use anchor-lang 0.29.0 only (no vendor) ([#40](https://github.com/drift-labs/drift-rs/pull/40))
- cargo fmt
- remove patch
- bump drift prog to 2.84.0
- Update liquidation.rs tests
- Reduce redundant requests from DriftClientBackend initialization ([#34](https://github.com/drift-labs/drift-rs/pull/34))
- ensure spot market account always included in AccountsMaps ([#33](https://github.com/drift-labs/drift-rs/pull/33))
- Add margin-category enum as param for get_collateral function ([#32](https://github.com/drift-labs/drift-rs/pull/32))
- Add function to get user's total and free collateral ([#30](https://github.com/drift-labs/drift-rs/pull/30))
- Remove async ([#27](https://github.com/drift-labs/drift-rs/pull/27))
- Add function to get user's margin requirements ([#26](https://github.com/drift-labs/drift-rs/pull/26))
- Fix clippy ([#24](https://github.com/drift-labs/drift-rs/pull/24))
- jit helpers/exports ([#22](https://github.com/drift-labs/drift-rs/pull/22))
- frank/jit infra ([#21](https://github.com/drift-labs/drift-rs/pull/21))
- frank/market & oracle maps ([#17](https://github.com/drift-labs/drift-rs/pull/17))
- copy jit-client across ([#19](https://github.com/drift-labs/drift-rs/pull/19))
- Fix/ws issues ([#20](https://github.com/drift-labs/drift-rs/pull/20))
- Allow create client w/out getProgramAccounts ([#15](https://github.com/drift-labs/drift-rs/pull/15))
- frank/dlob builder & leverage math ([#16](https://github.com/drift-labs/drift-rs/pull/16))
- frank/dlob ([#12](https://github.com/drift-labs/drift-rs/pull/12))
- frank/usermap ([#14](https://github.com/drift-labs/drift-rs/pull/14))
- frank/prelisting ([#13](https://github.com/drift-labs/drift-rs/pull/13))
- release ([#11](https://github.com/drift-labs/drift-rs/pull/11))
- Chore/crates setup ([#10](https://github.com/drift-labs/drift-rs/pull/10))
- Fix devnet txs ([#9](https://github.com/drift-labs/drift-rs/pull/9))
- frank/revert user stats change ([#7](https://github.com/drift-labs/drift-rs/pull/7))
- add missing accounts for place make/take ([#5](https://github.com/drift-labs/drift-rs/pull/5))
- add tx_idx to events
- program v2.61.0 update
- Merge branch 'isolate-sdk' of ../protocol-v2
- Initial commit

## [0.1.0](https://github.com/drift-labs/drift-rs/releases/tag/v0.1.0) - 2024-03-06

### Added
- add support for depth parameter in orderbook query ([#6](https://github.com/drift-labs/drift-rs/pull/6))
- feat/ci ([#1](https://github.com/drift-labs/drift-rs/pull/1))

### Other
- Chore/crates setup ([#10](https://github.com/drift-labs/drift-rs/pull/10))
- Fix devnet txs ([#9](https://github.com/drift-labs/drift-rs/pull/9))
- frank/revert user stats change ([#7](https://github.com/drift-labs/drift-rs/pull/7))
- add missing accounts for place make/take ([#5](https://github.com/drift-labs/drift-rs/pull/5))
- add tx_idx to events
- program v2.61.0 update
- Merge branch 'isolate-sdk' of ../protocol-v2
- Initial commit
