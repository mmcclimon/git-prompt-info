# git-prompt-info

This is a tiny Rust program to print out some information about git that my
dotfiles read for my prompt.

If you're not in a git dir at all, it prints `0` and exits.

If you are, it prints something like `1 on main 0`; the last number is 1 if
your tree is dirty, and if you're in a detached head it'll say something like
`1 at 67288c52 0` instead.

Then in my zsh files, I have this (where `$red` and `$teal` are color strings):

```zsh
function gprompt() {
    # is_gitdir at_or_on branch_or_sha is_dirty
    local gitinfo=( $(git prompt-info) )

    if [[ ${gitinfo[1]} -eq 0 ]] {
        return
    }

    print -n " ${gitinfo[2]} %F{$teal}${gitinfo[3]}"

    if [[ $gitinfo[4] -eq 1 ]] {
        print -n "%F{$red}*"
    }
}
```

Use it if you like, or don't; won't bother me either way!
