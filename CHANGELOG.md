# CHANGELOG

## [v0.4.0](https://github.com/Frazzer951/fpm/releases/tag/v0.4.0) - 2022-06-23 19:58:47

## What's Changed
* v0.4.0 Development by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/45


**Full Changelog**: https://github.com/Frazzer951/fpm/compare/v0.3.0...v0.4.0

### Feature

- general:
  - add options to print what files are getting moved ([9465dbe](https://github.com/Frazzer951/fpm/commit/9465dbe91ecbd99b27c9f51a9b07f9dd4d19d786)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Add ability to move and copy folders recursively ([b7b431a](https://github.com/Frazzer951/fpm/commit/b7b431a0b0495808edeffc7e880d5273dde4338d)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Add options to remove category and type when editing ([22ebb43](https://github.com/Frazzer951/fpm/commit/22ebb4300a58fcc837e7512b7fe02ad4061716a7)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Add options to verify to determin operation ([d71d60e](https://github.com/Frazzer951/fpm/commit/d71d60e0fb864204012629e047bfd5405a45d263)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - add command to open config dir ([1fe6846](https://github.com/Frazzer951/fpm/commit/1fe684689d41bd0e4ecfc507e824bd27c930f0fd)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - add command to add folders from a directory ([a64b06e](https://github.com/Frazzer951/fpm/commit/a64b06e2433dcc27e508a67a652e4aa99ab271c2)) ([#45](https://github.com/Frazzer951/fpm/pull/45))

### Bug Fixes

- general:
  - Fix refactor to be able to move .git folders ([54c938a](https://github.com/Frazzer951/fpm/commit/54c938ac36fa4c1501d5752a5edbd28346a75ba0)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Fix the move with refactor to not place the folder inside a folder with its own name ([a90c7a2](https://github.com/Frazzer951/fpm/commit/a90c7a25ee00b88df62f64cc9435783a574703cd)) ([#45](https://github.com/Frazzer951/fpm/pull/45))

### Continuous Integration

- general:
  - Remove unessecary WIP workflow ([097a238](https://github.com/Frazzer951/fpm/commit/097a2380f8090c59505a96eca2604d4028a7d0c4)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - update dependabot config ([462c706](https://github.com/Frazzer951/fpm/commit/462c706df09a8e12509c88728362deede6373a15)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Format workflows and add devlop branch to testing ([bf3a984](https://github.com/Frazzer951/fpm/commit/bf3a984415242dfb314a3c3743858ca65e04cdc1)) ([#45](https://github.com/Frazzer951/fpm/pull/45))

### Chore

- general:
  - Refactor Readme ([a242cb6](https://github.com/Frazzer951/fpm/commit/a242cb6e1e1be01bc79c1f3359154da85820856b)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - update dependecies ([a5a500f](https://github.com/Frazzer951/fpm/commit/a5a500f0373b9aa800082118c8cb408530231aa0)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Cargo Version to 0.4.0 ([a3f07b2](https://github.com/Frazzer951/fpm/commit/a3f07b26a349c03b9ededcb5da4aa35bf94b57c0)) ([#45](https://github.com/Frazzer951/fpm/pull/45))
  - Update Contributing section in readme ([dcb665e](https://github.com/Frazzer951/fpm/commit/dcb665e9e20d6de2658985a49da7bee18d930326))
  - Update Git Ignore ([69bf09b](https://github.com/Frazzer951/fpm/commit/69bf09bc8a0bc1b1f90a7a402904406a25d5916f))
  - Remove Cargo Lock ([996c04e](https://github.com/Frazzer951/fpm/commit/996c04e22a39687a64801886a13cff3666ca3d96))

## [v0.3.0](https://github.com/Frazzer951/fpm/releases/tag/v0.3.0) - 2022-06-22 03:15:28

## What's Changed
* feat: Create a project using a git url by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/39
* feat: verify projects by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/40
* Create LICENSE by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/41
* Damerau levenshtein dist by @Frazzer951 in https://github.com/Frazzer951/fpm/pull/42

## New Contributors
* @Frazzer951 made their first contribution in https://github.com/Frazzer951/fpm/pull/39

**Full Changelog**: https://github.com/Frazzer951/fpm/compare/v0.2.0...v0.3.0

### Feature

- general:
  - Add option to edit existing projects - closes #32 ([4998b73](https://github.com/Frazzer951/fpm/commit/4998b73fb3606fe576206a3456080fea1c0be83b))
  - add parent folders for file pointers ([20c0a45](https://github.com/Frazzer951/fpm/commit/20c0a4517952a8a01a81cdf1168616a3729a699b))
  - add user definable template variables - closes #38 ([c7151f2](https://github.com/Frazzer951/fpm/commit/c7151f201923fa2257bfce7804032a190e8ef040))
  - Add Project refactoring closes #31 ([101d715](https://github.com/Frazzer951/fpm/commit/101d71511354910b082eefda6e7fed88ad0fb71e))
  - Add Project refactoring closes #31 ([f283b44](https://github.com/Frazzer951/fpm/commit/f283b44493953dca04330a2485df24a5b4a974ac))
  - add open flag to new project closes #33 ([37e310c](https://github.com/Frazzer951/fpm/commit/37e310ce772e9991198211d224f401283cbfba37))
  - Verify recommends possible misspelled projects ([35853df](https://github.com/Frazzer951/fpm/commit/35853dfee0d060e1e913565a048a3e699fb4c9b5)) ([#42](https://github.com/Frazzer951/fpm/pull/42))
  - get similar project names ([f8e83ef](https://github.com/Frazzer951/fpm/commit/f8e83efc0e167148230d8c9b09b10607a3ab1be4)) ([#42](https://github.com/Frazzer951/fpm/pull/42))
  - verify projects ([d7fb0b8](https://github.com/Frazzer951/fpm/commit/d7fb0b831f42606f78a11eddc6b4211485bf96e6)) ([#40](https://github.com/Frazzer951/fpm/pull/40))
  - Create a project using a git url ([738620e](https://github.com/Frazzer951/fpm/commit/738620ebab8291024addf525a8ccd16f12811658)) ([#39](https://github.com/Frazzer951/fpm/pull/39))

### Bug Fixes

- general:
  - Fix error with user input when updating path in verify function ([5b1bdfc](https://github.com/Frazzer951/fpm/commit/5b1bdfcde9d5364e498bf672ced494f413de6dd2))

### Refactor

- general:
  - make project directory a global argument, so it can be parsed earlier ([b9b97d5](https://github.com/Frazzer951/fpm/commit/b9b97d5a9060f427aa066645dcb8a33ca7ca801e))
  - move list from a subcommand of project, to a base subcommand ([7f580d1](https://github.com/Frazzer951/fpm/commit/7f580d1c37c24fab1f021f472d6313ce2b53ac69))
  - move project functions into project.rs ([10b9035](https://github.com/Frazzer951/fpm/commit/10b9035e58c717d6ae3f1d9481884a0db667262c))
  - Extract sub commands into separate methods ([2019c46](https://github.com/Frazzer951/fpm/commit/2019c46119a9506930fbbf7535ac712af2ecdbcb))

### Chore

- general:
  - Add Comments ([8c313ff](https://github.com/Frazzer951/fpm/commit/8c313ff89c9852ba6cf821e4f8ec6d30edd485e0))
  - Update FPM Version to 0.3.0 ([1a27e85](https://github.com/Frazzer951/fpm/commit/1a27e85513b816fb7e99b72ba9eaf30c82ddc013))
  - remove roadmap from README.md ([a2f2d0f](https://github.com/Frazzer951/fpm/commit/a2f2d0fd66f9bbc969c7f9be0328c06d992a3feb))
  - Center Shields ([54e5205](https://github.com/Frazzer951/fpm/commit/54e520535e270ad5c355908abd735d801e021b08))
  - Update README.md ([417ccd3](https://github.com/Frazzer951/fpm/commit/417ccd3e5fe9c9e75ac8f21e9769cfe4d1dbef0a))
  - Update README.md ([fd09439](https://github.com/Frazzer951/fpm/commit/fd0943901d3f9c4cd423c33673ecb2a67f491d36))

## [v0.2.0](https://github.com/Frazzer951/fpm/releases/tag/v0.2.0) - 2022-06-16 21:11:22

## What's Changed
* :arrow_up: Bump clap from 3.1.18 to 3.2.3 by @dependabot in https://github.com/Frazzer951/fpm/pull/29
* :arrow_up: Bump actions/checkout from 2 to 3 by @dependabot in https://github.com/Frazzer951/fpm/pull/27


**Full Changelog**: https://github.com/Frazzer951/fpm/compare/v0.1.0...v0.2.0

### Feature

- general:
  - use specified template directory ([f4dbd29](https://github.com/Frazzer951/fpm/commit/f4dbd2967feb9e0241f279ddd0e27af2c4d2242b))
  - Add config setting to set a template directory ([f841d87](https://github.com/Frazzer951/fpm/commit/f841d87364e43e755edb99ddb4af1f7fb8aa6b6f))

### Bug Fixes

- general:
  - fix all the errors shown by clippy ([6119ae8](https://github.com/Frazzer951/fpm/commit/6119ae87968aaa5eb503cdd624dff18da1ef6e10))
  - fix clippy errors on un-needed muts ([43f2ae8](https://github.com/Frazzer951/fpm/commit/43f2ae8dec72dc945dbc9609701278295923d089))
  - clone project to solve missing traits ([103faa0](https://github.com/Frazzer951/fpm/commit/103faa0a3c822c851dbc05330bc4a8008c410947))
  - Fix Release action to use the correct binary ([bbb2508](https://github.com/Frazzer951/fpm/commit/bbb25084d6d071ddf5cf3730dac53d6a25d0c715))

### Refactor

- general:
  - Fix rustfmt.toml and format ([c00d41a](https://github.com/Frazzer951/fpm/commit/c00d41a14af209d3812e6be544bf18e70b6367aa))
  - Switch from Clap Derive API to Builder API ([7aff2b9](https://github.com/Frazzer951/fpm/commit/7aff2b9d38c2ba81a44697c2aa32c972e43b7a67))
  - extract main match statement into functions ([c950ad8](https://github.com/Frazzer951/fpm/commit/c950ad8508fcef88d994b2e98a7ff6ec0800e5e4))

### Continuous Integration

- general:
  - Fix action names ([78deca5](https://github.com/Frazzer951/fpm/commit/78deca5959e49e63c65a04fc7d9b8b005cf60811))
  - change dependabot commit prefix ([32ecf93](https://github.com/Frazzer951/fpm/commit/32ecf935d4858006da8bbba0377eda2489844776))

### Chore

- general:
  - Update packages ([754f750](https://github.com/Frazzer951/fpm/commit/754f750d378c37a47e8d801eb456ccbb4e119368))
  - Remove old templates ([2b81833](https://github.com/Frazzer951/fpm/commit/2b81833b024df06e334e27f978810696256a9e77))
  - Update issue templates ([faa015f](https://github.com/Frazzer951/fpm/commit/faa015f25bf122476982a408f589615250653f92))
  - Update issue templates ([f7c653b](https://github.com/Frazzer951/fpm/commit/f7c653b8a73e3d9563e4c6c59375dbfff40c6b10))
  - change cargo version number ([409f2e6](https://github.com/Frazzer951/fpm/commit/409f2e6d74f67f03bc5442207a834e211ed8a324))

## [v0.1.0](https://github.com/Frazzer951/fpm/releases/tag/v0.1.0) - 2022-06-13 23:38:44

## What's Changed
* Cargo Fmt and Clippy Fixes by @github-actions in https://github.com/Frazzer951/fpm/pull/1
* :arrow_up: Bump github/codeql-action from 1 to 2 by @dependabot in https://github.com/Frazzer951/fpm/pull/6

## New Contributors
* @github-actions made their first contribution in https://github.com/Frazzer951/fpm/pull/1
* @dependabot made their first contribution in https://github.com/Frazzer951/fpm/pull/6

**Full Changelog**: https://github.com/Frazzer951/fpm/commits/v0.1.0

### Bug Fixes

- general:
  - fix template alias ([5cde4ad](https://github.com/Frazzer951/fpm/commit/5cde4adff3b45292d0eb914117796da143679fbf))
  - fix formatting ([1bfb1a6](https://github.com/Frazzer951/fpm/commit/1bfb1a67254138c2033cd1c1372dd8026033ef3d))

### Documentation

- general:
  - Add config dir directions ([3506982](https://github.com/Frazzer951/fpm/commit/3506982633d4b1d3cc886cb3783c14424b587db5))

### Refactor

- general:
  - Rename config to Settings and move it into its own file ([daf0b38](https://github.com/Frazzer951/fpm/commit/daf0b386af4e2ffbbe596fd080b664cd155a4eb4))

### Continuous Integration

- general:
  - Add auto-changelog generation on release ([6492ea5](https://github.com/Frazzer951/fpm/commit/6492ea5e24e1071720f665cb64261c8018b41d70))

\* *This CHANGELOG was automatically generated by [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog)*
