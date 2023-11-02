# [1.9.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.8.0...v1.9.0) (2023-11-02)


### Features

* **cli:** :sparkles: Add `token` command to generate a sample token contract ([59124e5](https://github.com/SpiralOutDotEu/zk_whitelist/commit/59124e585d476e4d9934fb887c3c7948237375de))
* **cli:** :sparkles: Add command `all` to run all the other commands end-2-end ([25c0886](https://github.com/SpiralOutDotEu/zk_whitelist/commit/25c08866e71a36697612a3db51a5fc69676bec1c))
* **cli:** :sparkles: Include `token` command on `all` command ([9501075](https://github.com/SpiralOutDotEu/zk_whitelist/commit/9501075b55dfb5a401e43764d1f217c33faff741))

# [1.8.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.7.0...v1.8.0) (2023-11-01)


### Bug Fixes

* **cli:** :bug: use `IsEqual` template to correctly compare the addresses ([94f382a](https://github.com/SpiralOutDotEu/zk_whitelist/commit/94f382a9153b5058331aec05b600b056f6537d3c))


### Features

* **cli:** :sparkles: Add operations for read lines and write file ([6b068a7](https://github.com/SpiralOutDotEu/zk_whitelist/commit/6b068a7f88129dabf1494af1478dc987cfe154c8))
* **cli:** :sparkles: Implement procedure for creating proofs ([8651b81](https://github.com/SpiralOutDotEu/zk_whitelist/commit/8651b812f86cede11d245cf37aa289350dd4d55e))
* **cli:** :sparkles: Include `proofs` command ([56c20b5](https://github.com/SpiralOutDotEu/zk_whitelist/commit/56c20b56074516f24edf9c2b1daec2544c3fd7c8))

# [1.7.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.6.0...v1.7.0) (2023-10-31)


### Features

* **cli:** :sparkles: Add `movejs` command and assistant file operations ([64af834](https://github.com/SpiralOutDotEu/zk_whitelist/commit/64af8349c08ff91989065c0f3b862f3e284f01e2))

# [1.6.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.5.0...v1.6.0) (2023-10-30)


### Features

* **cli:** :sparkles: Add verifier command and tests ([16fc8db](https://github.com/SpiralOutDotEu/zk_whitelist/commit/16fc8dba161a6a9ed38200591509d82f25923bd0))

# [1.5.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.4.0...v1.5.0) (2023-10-30)


### Features

* **cli:** :sparkles: Add circuit module with copy_circuit_file function ([e010ba0](https://github.com/SpiralOutDotEu/zk_whitelist/commit/e010ba01d7aa7ddc42cd0a12c6c00bfead5f3502))
* **cli:** :sparkles: Add compile module with compile_circuit function ([d3f0cbf](https://github.com/SpiralOutDotEu/zk_whitelist/commit/d3f0cbfb4b76608aece920dbdb6a7874a2d820f8))
* **cli:** :sparkles: Add setup module with execute_setup_command and ensure_success functions ([bfa60e9](https://github.com/SpiralOutDotEu/zk_whitelist/commit/bfa60e9ccbf304e497196710b92d6b7f46d48333))


### Reverts

* **cli:** :rewind: revert the complext circuit execution ([e8cca4e](https://github.com/SpiralOutDotEu/zk_whitelist/commit/e8cca4ebc826dfa88119ad273f78a2644e75dba8))

# [1.4.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.3.1...v1.4.0) (2023-10-30)


### Features

* **cli:** :sparkles: Implement `setup` command that runs a ceremony and generates required files ([2dd7951](https://github.com/SpiralOutDotEu/zk_whitelist/commit/2dd79518e847a038c7763b1dbe2ad1a841ad3692))

## [1.3.1](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.3.0...v1.3.1) (2023-10-30)


### Bug Fixes

* **cli:** :bug: correct short version option test ([5b88725](https://github.com/SpiralOutDotEu/zk_whitelist/commit/5b88725f0019be1e9774446f41fa21c7028a4d5a))

# [1.3.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.2.1...v1.3.0) (2023-10-29)


### Features

* **cli:** :building_construction: Migrate CLI Argument Parsing to Clap ([09d03e0](https://github.com/SpiralOutDotEu/zk_whitelist/commit/09d03e0de6648f463b7006806fd49a09d1b22b2f))

## [1.2.1](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.2.0...v1.2.1) (2023-10-29)


### Bug Fixes

* **ci:** :bug: Add circom path and run test on push ([9aad534](https://github.com/SpiralOutDotEu/zk_whitelist/commit/9aad534a158d8558d0bbdc889cf4b071b1c9a8bc))
* **ci:** :bug: Ignore errors on circom build ([4e85c3b](https://github.com/SpiralOutDotEu/zk_whitelist/commit/4e85c3ba7081ccf59b9ca741c0ea83c6558f6144))
* **ci:** :bug: Update GitHub Actions to install circom and snarkjs ([4b87618](https://github.com/SpiralOutDotEu/zk_whitelist/commit/4b87618d181ce684977e7ac942450b1d315719bf))

# [1.2.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.1.0...v1.2.0) (2023-10-29)


### Bug Fixes

* **cli:** :bug: Make public variable in array ([88b5313](https://github.com/SpiralOutDotEu/zk_whitelist/commit/88b53137dc9f8c60e04b09ac9608ffc0f5add905))


### Features

* **cli:** :sparkles: Implement `compile` command to compile the circuit file ([a81d213](https://github.com/SpiralOutDotEu/zk_whitelist/commit/a81d213727d761ca33bd11105ea18a87382f733a))

# [1.1.0](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.10...v1.1.0) (2023-10-29)


### Features

* **cli:** :sparkles: Add `circuit` command to create `circuit.circom` on user's folder ([943931b](https://github.com/SpiralOutDotEu/zk_whitelist/commit/943931b8eaf1e30576a0dfb354524d637ac6aaa1))

## [1.0.10](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.9...v1.0.10) (2023-10-28)


### Bug Fixes

* **ci:** :bug: Add Cargo.lock to auto git files ([9131659](https://github.com/SpiralOutDotEu/zk_whitelist/commit/913165983ec451e80d5e7f1b94e659573f93fbab))

## [1.0.9](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.8...v1.0.9) (2023-10-28)


### Bug Fixes

* **ci:** :bug: Update the version in Cargo.lock with update_version script ([db6ee49](https://github.com/SpiralOutDotEu/zk_whitelist/commit/db6ee49c938a516f6a3ba7984dae696d976414e7))

## [1.0.8](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.7...v1.0.8) (2023-10-28)


### Bug Fixes

* **ci:** :bug: Sync cargo.lock version ([1cd7d32](https://github.com/SpiralOutDotEu/zk_whitelist/commit/1cd7d327c86a644aab2760c1911cf7e8a8b41e27))

## [1.0.7](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.6...v1.0.7) (2023-10-28)


### Bug Fixes

* **ci:** :bug: Add build into update version script ([b384deb](https://github.com/SpiralOutDotEu/zk_whitelist/commit/b384deb43c67ef22f28a959857026f2c82f0d7d1))

## [1.0.6](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.5...v1.0.6) (2023-10-28)


### Bug Fixes

* **ci:** :bug: merge build and semantic version run commands ([90a90d0](https://github.com/SpiralOutDotEu/zk_whitelist/commit/90a90d07f2855cd7b9806905c376dc283c8d1516))

## [1.0.5](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.4...v1.0.5) (2023-10-28)


### Bug Fixes

* **ci:** :bug: manual upload release binary ([84dd40a](https://github.com/SpiralOutDotEu/zk_whitelist/commit/84dd40ad2b2356cfcb45c7872545e09739eff9e1))

## [1.0.4](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.3...v1.0.4) (2023-10-28)


### Bug Fixes

* **ci:** :bug: change order of release build ([97f9dd2](https://github.com/SpiralOutDotEu/zk_whitelist/commit/97f9dd2295646dae32d2b8d1fbafc11380a97328))

## [1.0.3](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.2...v1.0.3) (2023-10-28)


### Bug Fixes

* **ci:** :bug: update cargo.lock after version update ([004747e](https://github.com/SpiralOutDotEu/zk_whitelist/commit/004747e77d159a54bfde45a669aacd3595b7aee3))

## [1.0.2](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.1...v1.0.2) (2023-10-28)


### Bug Fixes

* **ci:** :bug: sync cargo.lock version ([12b7fd1](https://github.com/SpiralOutDotEu/zk_whitelist/commit/12b7fd1580da0c3865f73b176ab1464397918b25))

## [1.0.1](https://github.com/SpiralOutDotEu/zk_whitelist/compare/v1.0.0...v1.0.1) (2023-10-28)


### Bug Fixes

* **ci:** :bug: change order of release actions ([4eee730](https://github.com/SpiralOutDotEu/zk_whitelist/commit/4eee73057a1aa0522c3289c97bd2fe79544dbf94))

# 1.0.0 (2023-10-28)


### Bug Fixes

* **ci:** :bug: Add  permissions:  contents: write ([aa15f8f](https://github.com/SpiralOutDotEu/zk_whitelist/commit/aa15f8f2e151ff79fccfa0f22404e67dca39a986))
* **ci:** :bug: Add GH_TOKEN env variable ([00e170b](https://github.com/SpiralOutDotEu/zk_whitelist/commit/00e170be532389585d023486fd7234de1adae025))
* **ci:** :bug: add missing dependencies ([d4e9c49](https://github.com/SpiralOutDotEu/zk_whitelist/commit/d4e9c4995b3e77e72605e23264125df027ed4b63))
* **ci:** :bug: add repo on package and change order of release commands ([8044721](https://github.com/SpiralOutDotEu/zk_whitelist/commit/8044721b8f384e6635ed722a53994051398a10c8))
* **ci:** :bug: correct master branch name ([78e61a8](https://github.com/SpiralOutDotEu/zk_whitelist/commit/78e61a89e0f914b83450745a55ab35ea529315c4))
* **ci:** :bug: Fix branches definition ([094be0f](https://github.com/SpiralOutDotEu/zk_whitelist/commit/094be0f08a720a9223aec80104ecf8dea97e3a6f))
* **ci:** :bug: use newer node version ([f460bea](https://github.com/SpiralOutDotEu/zk_whitelist/commit/f460beae4e18771bf71753ea83f9839fe1505bb4))
* **ci:** :bug: use newer rust action and node version ([32dbefa](https://github.com/SpiralOutDotEu/zk_whitelist/commit/32dbefaa3e7770156ceb8413ed3e7133e3110fbb))
* **ci:** :bug: use newer rust toolchain ([b0a6439](https://github.com/SpiralOutDotEu/zk_whitelist/commit/b0a64390f911d12f0b692698016eb7f01faa01a2))
* **ci:** :bug: use personal token ([dde2db2](https://github.com/SpiralOutDotEu/zk_whitelist/commit/dde2db2a271b75deaa2ba7280a26110307cf166f))


### Features

* :tada: Init ([af22afa](https://github.com/SpiralOutDotEu/zk_whitelist/commit/af22afab2c74ce0fb2774546f6d34e30c38fbabd))
* **ci:** :sparkles: Add semantic-release automation for versioning ([30fb154](https://github.com/SpiralOutDotEu/zk_whitelist/commit/30fb154df3747261f6624462dd57468244954087))
* **cli:** :sparkles: Add --version and --v commands ([ae42c86](https://github.com/SpiralOutDotEu/zk_whitelist/commit/ae42c866a6b7b7d974af4ef59009fd201de2748d))
