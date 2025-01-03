# Mega-merges (merge workflow)

Keeping multiple parallel streams of work running,
with the main checkout combining them all.

## Start

Create a merge commit with multiple parents:

```
jj new c n x
```

Mark this merge commit:

```
jj commit -m '[merge]'
```

Mark the `wip` commit:

```
jj describe -m '[wip]'
```

All work should happen on this commits (or commits after that).

## Squash into one of the workstreams

To squash things back into the correct workstream:

```
jj squash --into n --keep-emptied
```

where `n` is the latest commit of that workstream.

`--keep-emptied` keeps the `wip` commit after the merge.

## Adding a new workstream

To create a new arm of the merge:

```
jj new m
```

where `m` is the commit that points to the common ancestor, e.g. the `main` branch.

Then rebase the merge on top:

```
jj rebase -s orl -d "all:orl-" -d wtm
```

Rebase `orl` and its descendants on top of:

* all of `orl`'s existing parents (the previous merge parents)
* and `wtm`

---

Most of this workflow I got from the following sources:

* [A Better Merge Workflow with Jujutsu](https://ofcr.se/jujutsu-merge-workflow)
* [Jujutsu Megamerges and jj absorb](https://v5.chriskrycho.com/journal/jujutsu-megamerges-and-jj-absorb/)
