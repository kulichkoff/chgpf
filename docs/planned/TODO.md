# Planned Features

This document describes the features I'd like to implement within `chgpf`.

## Init Command

- **Status**: supported

```bash
chgpf init
```

This command should:

- create starter config
- apply default profile like current config

## List Available Profiles

- **Status**: supported

Profiles listing might be useful if a user has more Git configurations than usual.

```bash
# Call the command
chgpf list

# Sample output
[home]
name = "Daniel Kulichkov"
email = "daniel@example.com"
---------
[work]
name = "Daniel Kulichkov"
email = "dkulichkov@job.org"
```

## Show Active Profile

- **Status**: planned

Do you always remember about `git config user.email`?

```bash
# Call the command
chgpf current

# Sample output
[home]
name = "Daniel Kulichkov"
email = "daniel@example.com"
```

## Profile Inheritance

- **Status**: planned

Do you always remember about `git config user.email`?

```toml
[base]
name = "John Doe"

[work]
extends = "base"
user = "work@example.com"
```

## Interactive Selector

- **Status**: planned

Example:

```bash
chgpf
```

Then show:

- fuzzy search
- arrow navigation
- interactive picker

This would make the tool feel polished.
