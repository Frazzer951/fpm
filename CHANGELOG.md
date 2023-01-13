# Changelog

## [unreleased]

### Bug Fixes

- Make sure path to database exists before trying to connect

### Continuous Integration

- Conditional steps
- Fix if statements in release.yaml
- Remove brackets from release.yaml
- Use proper extension
- Enforce more clippy checks, closes #59
- Update changelog action
- Add Manual dispatch of changelog generation
- Update git-cliff-action to v2

### Documentation

- Update TOC
- Update CLI Readme
- Update ToDo's

### Features

- Add Project Struct
- Add custom error enum
- Load Project Database
- Default Config
- Save database to file
- Start of cli to create project
- Load and Save config
- Start of creating new project
- Create Project Folder
- Create project folder and add it to the database
- Basic implementation of `list`
- Prettify printing of project list
- Reset Database

### Miscellaneous Tasks

- Setup more modules
- Add rustfmt.toml
- Update readme with resources

### Other

- Update changelog generation


### Refactor

- Rename crates
- Rewrite database.rs to use an SQLite DB
- No longer create new config if one doesnt already exist
- Cleanup New Project Function
- Move all command into own module to improve readability

### Testing

- Setup infra for testing if files are equal
- Save and Load empty database
- Fix error in test
- Add prints to debug testing

## [0.6.0] - 2022-11-12

### Bug Fixes

- Fix default git command - closes #61
- Fix clippy errors

### Continuous Integration

- Set specific version
- Make changelog.yml callable
- Update CIs
- Update release yaml to be more generic
- Fix bug in release.yaml and remove MacOS support

### Documentation

- Update release notes

### Features

- Ability to set database directory

### Miscellaneous Tasks

- Update serial_test requirement from 0.8.0 to 0.9.0
- Bump actions/checkout from 2 to 3
- Update dependencies
- Update dependencies
- Set toolchain and bump version

### Other

- PR # [63](https://github.com/Frazzer951/fpm/pull/63): bump actions/checkout from 2 to 3

### Refactor

- Project Rewrite

## [0.5.0] - 2022-08-03

### Continuous Integration

- Set `auto-generate-changelog` version
- Add codecov action
- Manually run cargo-tarpaulin
- Fix codecov.yml
- Codecov.yml add workflow dispatch
- Codecov.yml fix cache
- Add `test` type to Types
- Use recommended Coverage workflow
- Add config file for tarpaulin
- Set toolchain to nightly
- Run changelog.yml on cron too
- Use the nightly toolchain

### Features

- Add flag to refactor after editing - closes #50
- Add template groups and refactor how templates are processed - closes #49

### Miscellaneous Tasks

- Remove un-needed feature
- Update serde_yaml requirement from 0.8.24 to 0.9.0
- Add Codecov badge to readme
- Set crate version

### Other

- PR # [52](https://github.com/Frazzer951/fpm/pull/52): refactor on edit
- PR # [53](https://github.com/Frazzer951/fpm/pull/53): cleanup cli and move it to another file
- PR # [54](https://github.com/Frazzer951/fpm/pull/54): Move CLI stuff to another file
- PR # [56](https://github.com/Frazzer951/fpm/pull/56): Add template groups and refactor how templates are processed - …
- PR # [57](https://github.com/Frazzer951/fpm/pull/57): update serde_yaml requirement from 0.8.24 to 0.9.0
- PR # [58](https://github.com/Frazzer951/fpm/pull/58): Tests
- PR # [55](https://github.com/Frazzer951/fpm/pull/55): Development 

### Refactor

- Move CLI stuff to another file
- Move bulk of code into lib.rs for testing
- Refactor projects to use a custom struct

### Testing

- Verify default git command
- Add more tests for settings.rs
- Add more tests for settings.rs
- Add basic load test for Projects
- Add more tests for Projects
- Test add_project
- Don't check tests dir for coverage

## [0.4.1] - 2022-07-20

### Bug Fixes

- Fix bug that causes crash when no config file exists

### Documentation

- Update release notes

### Features

- Add ability to set git_command from commandline

### Miscellaneous Tasks

- Set version to dev
- Remove dev from version

### Other

- PR # [48](https://github.com/Frazzer951/fpm/pull/48): Development

## [0.4.0] - 2022-06-23

### Bug Fixes

- Fix the move with refactor to not place the folder inside a folder with its own name
- Fix refactor to be able to move .git folders

### Continuous Integration

- Format workflows and add devlop branch to testing
- Update dependabot config
- Remove unessecary WIP workflow

### Documentation

- Update release notes

### Features

- Add command to add folders from a directory
- Add command to open config dir
- Add options to verify to determin operation
- Add options to remove category and type when editing
- Add ability to move and copy folders recursively
- Add options to print what files are getting moved

### Miscellaneous Tasks

- Remove Cargo Lock
- Update Git Ignore
- Update Contributing section in readme
- Cargo Version to 0.4.0
- Update dependecies
- Refactor Readme

### Other

- WIP workflow
- Use fs_err instead of fs

- PR # [45](https://github.com/Frazzer951/fpm/pull/45): Development

## [0.3.0] - 2022-06-22

### Bug Fixes

- Fix error with user input when updating path in verify function

### Documentation

- Update release notes
- Update release notes
- Update release notes

### Features

- Create a project using a git url
- Verify projects
- Get similar project names
- Verify recommends possible misspelled projects
- Add open flag to new project closes #33
- Add Project refactoring closes #31
- Add Project refactoring closes #31
- Add user definable template variables - closes #38
- Add parent folders for file pointers
- Add option to edit existing projects - closes #32

### Miscellaneous Tasks

- Update README.md
- Update README.md
- Center Shields
- Remove roadmap from README.md
- Update FPM Version to 0.3.0
- Add Comments

### Other

- PR # [39](https://github.com/Frazzer951/fpm/pull/39): Create a project using a git url
- PR # [40](https://github.com/Frazzer951/fpm/pull/40): verify projects
- Create LICENSE
- PR # [41](https://github.com/Frazzer951/fpm/pull/41): LICENSE
- PR # [42](https://github.com/Frazzer951/fpm/pull/42): levenshtein dist
- Merge remote-tracking branch 'origin/main'

# Conflicts:
#	src/project.rs


### Refactor

- Extract sub commands into separate methods
- Move project functions into project.rs
- Move list from a subcommand of project, to a base subcommand
- Make project directory a global argument, so it can be parsed earlier

## [0.2.0] - 2022-06-16

### Bug Fixes

- Fix Release action to use the correct binary
- Clone project to solve missing traits
- Fix clippy errors on un-needed muts
- Fix all the errors shown by clippy

### Continuous Integration

- Change dependabot commit prefix
- Fix action names

### Documentation

- Update release notes

### Features

- Add config setting to set a template directory
- Use specified template directory

### Miscellaneous Tasks

- Change cargo version number
- Update issue templates
- Update issue templates
- Remove old templates
- Update packages

### Other

- Merge remote-tracking branch 'origin/main'

- :arrow_up: Bump actions/checkout from 2 to 3

Bumps [actions/checkout](https://github.com/actions/checkout) from 2 to 3.
- [Release notes](https://github.com/actions/checkout/releases)
- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)
- [Commits](https://github.com/actions/checkout/compare/v2...v3)

---
updated-dependencies:
- dependency-name: actions/checkout
  dependency-type: direct:production
  update-type: version-update:semver-major
...

Signed-off-by: dependabot[bot] <support@github.com>
- :arrow_up: Bump clap from 3.1.18 to 3.2.3

Bumps [clap](https://github.com/clap-rs/clap) from 3.1.18 to 3.2.3.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/v3.1.18...v3.2.3)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>
- PR # [29](https://github.com/Frazzer951/fpm/pull/29): Bump clap from 3.1.18 to 3.2.3
- PR # [27](https://github.com/Frazzer951/fpm/pull/27): Bump actions/checkout from 2 to 3
- Update issue templates

### Refactor

- Extract main match statement into functions
- Switch from Clap Derive API to Builder API
- Fix rustfmt.toml and format

## [0.1.0] - 2022-06-13

### Bug Fixes

- Fix formatting

- Fix template alias


### Continuous Integration

- Add auto-changelog generation on release

### Documentation

- Add config dir directions

### Other

- First commit

- Add project files

- Start of project manager

- Parse template

- Working setup for c++

- Add CI files

- Run fixes

- Cargo Fmt and Clippy Fixes

- PR # [1](https://github.com/Frazzer951/fpm/pull/1): Fmt and Clippy Fixes
- Remove some tests

- Implement open and git flags

- Ability to specify templates

- Move template stuff to its own folder

- Move file operations into its own file

- Add comments

- Update packages

- Make readme optional

- Can include custom commands to run before creating folders and files

- Reorder some code

- Create rust-clippy.yml
- Update README.md

- Merge remote-tracking branch 'origin/main'

- Start of new config command

- Remove templates directory

- Add template submodule

- Remove config stuff for now

- :arrow_up: Bump github/codeql-action from 1 to 2

Bumps [github/codeql-action](https://github.com/github/codeql-action) from 1 to 2.
- [Release notes](https://github.com/github/codeql-action/releases)
- [Changelog](https://github.com/github/codeql-action/blob/main/CHANGELOG.md)
- [Commits](https://github.com/github/codeql-action/compare/v1...v2)

---
updated-dependencies:
- dependency-name: github/codeql-action
  dependency-type: direct:production
  update-type: version-update:semver-major
...

Signed-off-by: dependabot[bot] <support@github.com>
- Update dependabot

- Update dependabot config

- Rename project

- Rename Project

- Setup basic structure

- Very basic functionality

- PR # [6](https://github.com/Frazzer951/fpm/pull/6): Bump github/codeql-action from 1 to 2
- Format

- Update workflow and format

- Config Settings

- Remove test_offline from test.yml

- New an Improved GitHub Actions

- Fix ToolChain

- Save projects to a json file

- File Error Handling

- Error if folder isnt empty

- Refactoring

- Start of templates

- Process templates

- Remove duplicates when saving projects db

- Process file and folder pointers

- Specify multiple templates

- Format

- Ability to add projects

- List out known projects with a filter options

- Update README.md


### Refactor

- Rename config to Settings and move it into its own file

