# gcollab
Script for managing Git Collaborators

## Necessary Setup

* Create a directory in your home directory `.gcollab`
* add this directory to your $PATH variable
  * `export PATH="$HOME/.gcollab:$PATH"` in your `.zshrc` or `.bash`

## Installing

* Download latest [binary executable](https://github.com/Sparkmasterflex/gcollab/releases)
* move it to `~/.gcollab`

or

* [Install rustlang](https://www.rust-lang.org/tools/install)
    - or with homebrew: `brew install rustup`
* clone repo
* `cp target/release/gcollab ~/.gcollab`


## Commands

### Adding Collaborators

`$ gcollab add` - add a new github user to list of collaborators ready for copying.

With out any arguments gcollab will ask for:

| attribute | description |
|-----------|-------------|
| identifier | a short slug that will be used to find a collaborator for copying |
| full name | User's full name as displayed in GitHub |
| email | User's email as displayed in GitHub |

For quicker adding you can also pass the required arguments to the `add` command:

```
  $ gcollab add sparky "Sparky McSparkerson" sparky@someemail.com
```

Or you can create `collaborators.json` file in the `.gcollab` directory and build your json file yourself

```json
  [
    { "slug":"groucho", "name":"Groucho Marx", "email":"groucho@marx.com", "last_used":true },
    { "slug":"harpo", "name":"Harpo Marx", "email":"harpo@marx.com", "last_used":true },
    { "slug":"chico", "name":"Chico Marx", "email":"chico@marx.com", "last_used":true },
    { "slug":"zeppo", "name":"Zeppo Marx", "email":"zeppo@marx.com", "last_used":true }
  ]
```

### Copying Collaborators' Co-authored-by tag

```
  $ gcollab sparky
  > Co-authored-by: Sparky McSparkerson <sparky@someemail.com> copied!
```

### Listing Collaborators

Displays list of available git collaborator identifiers available. Use these to then run `gcollab <identifier>` to get their Co-author tag

```
  $ gcollab list
    sparky
    frankie
    bob
```

### Remove Collaborator

Remove a collaborator from your list by passing their identifier

```
  $ gcollab remove sparky
  > sparky removed!
```
