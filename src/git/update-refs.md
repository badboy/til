# Rebase dependent branches with `--update-refs`

To automatically adjust all intermediary branches of a larger patch stack rebase with `--update-refs` on the latest commit:

```
git rebase -i main --autosquash --update-refs
```

via [git 2.38 release notes](https://github.blog/2022-10-03-highlights-from-git-2-38/#rebase-dependent-branches-with-update-refs)
