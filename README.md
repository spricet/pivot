# pivot

### Enabling Post-Init Features
Add the following to your `~/.bashrc`
```bash
### Pivot ###
if [ ! -z "$PIVOT_PS1" ]; then
    PS1="$PIVOT_PS1"
fi
if [ ! -z "$PIVOT_START_DIR" ]; then
    cd $(eval "ls -d $PIVOT_START_DIR")
fi
### End Pivot ###
```