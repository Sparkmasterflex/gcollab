# gcollab
Script for managing Git Collaborators

## Installing

* Download [binary executable](https://github.com/Sparkmasterflex/gcollab/releases)
* move it to `/usr/local/bin/`

or

* [Install rustlang](https://www.rust-lang.org/tools/install)
    - or with homebrew: `brew install rustup`
* clone repo
* `cd` to local directory and run `cargo build --release`
* `cp target/release/npm_pub /usr/local/bin/`

_This is all assuming that `/usr/local/bin/` is in your `$PATH` variable_

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

### Copying Collaborators' Co-authored-by tag

```
  $ gcollab sparky
  > Co-authored-by: Sparky McSparkerson <sparky@someemail.com> copied!
```

### Listing Collaborators

Displays list of available git collaborator identifiers available. Use these to then run `gcollab <identifier>` to get their Co-author tag

```
  $ gcollab list
  > sparky
    frankie
    bob
```

### Remove Collaborator

Remove a collaborator from your list by passing their identifier

```
  $ gcollab remove sparky
  > sparky removed!
```
