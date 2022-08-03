# CHANGELOG

## [v0.5.0](https://github.com/Frazzer951/fpm/releases/tag/v0.5.0) - 2022-08-03 18:06:26

## What's Changed
* feat: refactor on edit by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/52
* refactor: cleanup cli and move it to another file by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/53
* refactor: Move CLI stuff to another file by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/54
* feat: Add template groups and refactor how templates are processed - … by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/56
* chore: update serde_yaml requirement from 0.8.24 to 0.9.0 by @dependabot in https://github.com/Frazzer951/fpm/pull/57
* Add Tests by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/58
* v0.5.0 Development  by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/55


**Full Changelog**: https://github.com/Frazzer951/fpm/compare/v0.4.1...v0.5.0

### Feature

- general:
  - Add template groups and refactor how templates are processed - closes #49 ([7cff400](https://github.com/Frazzer951/fpm/commit/7cff400b7f820f45b6bd440ffc707e177ba16d5d)) ([#56](https://github.com/Frazzer951/fpm/pull/56))
  - Add flag to refactor after editing - closes #50 ([876bcc0](https://github.com/Frazzer951/fpm/commit/876bcc097f496add1d961fe2c7835b505b92a722)) ([#52](https://github.com/Frazzer951/fpm/pull/52))

### Refactor

- general:
  - Refactor projects to use a custom struct ([64d7589](https://github.com/Frazzer951/fpm/commit/64d7589c90debbf0e7108904219472b69db83149)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - move bulk of code into lib.rs for testing ([b67117c](https://github.com/Frazzer951/fpm/commit/b67117c9d7192c87ea517b44bc7f901d56acbf69)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Move CLI stuff to another file ([bb4de4c](https://github.com/Frazzer951/fpm/commit/bb4de4c16814398fda8ae0b9c6bb948a23cf9da1)) ([#53](https://github.com/Frazzer951/fpm/pull/53))

### Continuous Integration

- general:
  - use the nightly toolchain ([e256e9f](https://github.com/Frazzer951/fpm/commit/e256e9fe0ca78328e8b55329aa5aa02113cdaeac))
  - run changelog.yml on cron too ([3e59c8a](https://github.com/Frazzer951/fpm/commit/3e59c8a71b6e5844b058bdbdf6106e7b2aa1ce1b))
  - set toolchain to nightly ([d9477c4](https://github.com/Frazzer951/fpm/commit/d9477c49ec9fbaccc21556395f81792e3f9c0b70)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - Add config file for tarpaulin ([e07a3ef](https://github.com/Frazzer951/fpm/commit/e07a3ef3af466d61c0e6c9f0d366724ccc49b950)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Use recommended Coverage workflow ([96af530](https://github.com/Frazzer951/fpm/commit/96af530639420166d00b4c4ba08be26bd008b79b)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Add `test` type to Types ([fa478e8](https://github.com/Frazzer951/fpm/commit/fa478e8f840ecc38cb50d447d728eaee50cdc587)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - codecov.yml fix cache ([60b0557](https://github.com/Frazzer951/fpm/commit/60b055793b67cda5c81f012feeee18e5ac0ee980)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - codecov.yml add workflow dispatch ([036a290](https://github.com/Frazzer951/fpm/commit/036a2908f6420d14cae46e69cfa2ccafeccd4ff1)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - fix codecov.yml ([cbe6c58](https://github.com/Frazzer951/fpm/commit/cbe6c58f148411fd6b43c9c82546733c6159fbdf)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - manually run cargo-tarpaulin ([685994a](https://github.com/Frazzer951/fpm/commit/685994ae16f89e7fd2e9398b17abe2c0f8ed3077)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - add codecov action ([0b85158](https://github.com/Frazzer951/fpm/commit/0b8515823242382725c8e52ea2644a309c40ff6e)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - set `auto-generate-changelog` version ([396422d](https://github.com/Frazzer951/fpm/commit/396422d37b093c49df0566d893e9fca1f1a7712e)) ([#52](https://github.com/Frazzer951/fpm/pull/52))

### Chore

- general:
  - set crate version ([7287d2d](https://github.com/Frazzer951/fpm/commit/7287d2d212f28de976c59795db45463175614f8c)) ([#55](https://github.com/Frazzer951/fpm/pull/55))
  - Add Codecov badge to readme ([aba36bd](https://github.com/Frazzer951/fpm/commit/aba36bd41e22bf9b83c0a3adcce2ad0b6a276d8b)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - update serde_yaml requirement from 0.8.24 to 0.9.0 ([bde307b](https://github.com/Frazzer951/fpm/commit/bde307b76cfe6bac22966577a36b64dae64b6e95)) ([#57](https://github.com/Frazzer951/fpm/pull/57))
  - remove un-needed feature ([b1b3a61](https://github.com/Frazzer951/fpm/commit/b1b3a61c9a126121dadd7d9f9ddd4294b54faefa)) ([#55](https://github.com/Frazzer951/fpm/pull/55))

### Tests

- general:
  - don't check tests dir for coverage ([320b0bb](https://github.com/Frazzer951/fpm/commit/320b0bb9d18b795f5776b60538969e5e956b67ad))
  - Test add_project ([9b4bfb8](https://github.com/Frazzer951/fpm/commit/9b4bfb82c0039ffd180cdd8a5a1c8ae8e1d02e39)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Add more tests for Projects ([63d2920](https://github.com/Frazzer951/fpm/commit/63d29208ebb7f67d9f3f30f92bac922fa4d2ee62)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Add basic load test for Projects ([754cbe8](https://github.com/Frazzer951/fpm/commit/754cbe8822ee8f9dc3eed38f7c3b2c73670905bc)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Add more tests for settings.rs ([3d12310](https://github.com/Frazzer951/fpm/commit/3d12310f0019ce1becee44b2dc1d86a531fe5ad6)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Add more tests for settings.rs ([1ecdc51](https://github.com/Frazzer951/fpm/commit/1ecdc510a48f9916f82de769beafeab23f8458c3)) ([#58](https://github.com/Frazzer951/fpm/pull/58))
  - Verify default git command ([159b2d2](https://github.com/Frazzer951/fpm/commit/159b2d20674494b7bb992cf0b4a7d8206ac9f331)) ([#58](https://github.com/Frazzer951/fpm/pull/58))

## [v0.4.1](https://github.com/Frazzer951/fpm/releases/tag/v0.4.1) - 2022-07-20 22:56:55

## What's Changed
* v0.4.1 Development by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/48


**Full Changelog**: https://github.com/Frazzer951/fpm/compare/v0.4.0...v0.4.1

### Feature

- general:
  - Add ability to set git_command from commandline ([ed9a3ef](https://github.com/Frazzer951/fpm/commit/ed9a3ef39ef2dc3533c648b6cdb2377e72a494df)) ([#48](https://github.com/Frazzer951/fpm/pull/48))

### Bug Fixes

- general:
  - Fix bug that causes crash when no config file exists ([9aa13bd](https://github.com/Frazzer951/fpm/commit/9aa13bd592868178a8553078b2078a45ceba0f04)) ([#48](https://github.com/Frazzer951/fpm/pull/48))

### Chore

- general:
  - remove dev from version ([672c8d3](https://github.com/Frazzer951/fpm/commit/672c8d3d25dc5330b5b6b921e6d2a1fbd0a6dc55)) ([#52](https://github.com/Frazzer951/fpm/pull/52))
  - set version to dev ([1727306](https://github.com/Frazzer951/fpm/commit/172730650c16581b4557e7ea61299be4e38564c8)) ([#48](https://github.com/Frazzer951/fpm/pull/48))

## [v0.4.0](https://github.com/Frazzer951/fpm/releases/tag/v0.4.0) - 2022-06-23 19:58:47

## [v0.3.0](https://github.com/Frazzer951/fpm/releases/tag/v0.3.0) - 2022-06-22 03:15:28

## [v0.2.0](https://github.com/Frazzer951/fpm/releases/tag/v0.2.0) - 2022-06-16 21:11:22

## [v0.1.0](https://github.com/Frazzer951/fpm/releases/tag/v0.1.0) - 2022-06-13 23:38:44

\* *This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)*
