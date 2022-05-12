# Fixup commits

Fixup commits are commits that build on top of an already existing commit.
They can be squashed into the existing commit as a later fixup, e.g. to fix typos or formatting.

`git commit` comes with builtin support for that: `git commit --fixup=<commit>`, 
where `<commit>` is the existing commit to be modified.
See [the documentation for details][man].

See also [git helpers][helpers].

[man]: https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---fixupamendrewordltcommitgt
[helpers]: git-helpers.md
