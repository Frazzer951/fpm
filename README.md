# Project Manager

A CLI app to create and manage projected

## Usage

`project_manager.exe <COMMAND>`

### Commands:

| Command | Description                    |
|---------|--------------------------------|
| Create  | Create a New project directory |

## Create

### Usage

`project_manager create [OPTIONS] --language <LANGUAGE> <NAME>`

| Option              | Description                                                 | Default          |
|---------------------|:------------------------------------------------------------|------------------|
| -b, --base-dir      | The base directory to place the project folders into        | Current Diectory |
| -g, --git-repo      | Create a git repo for the project                           |                  |
| -h, --help          | Print help information                                      |                  |
| -l, --language      | The language that the project will use                      |                  |
| -n, --template-name | Specify specific template, leave blank for default template | Language         |
| -o, --open          | Open the folder when done                                   |                  |
| -r, --readme        | Include a basic README.md file                              |                  |
| -t, --template      | Whether to use the template for the language                |                  |
| -V, --version       | Print version information                                   |                  |
