```
waypoint . # stores current directory as last remembers portable waypoint

cd testing
cd integration

waypoint back # restores current working directory as last placed portable waypoint
```

## Installation
> [!WARNING]
> Currently only supports bash. Support for other shells is in progress

```sh
# Add this to your .bashrc
function _waypoint__() {
  local result

  if [ "$1" = "back" ]; then
    result=$(waypoint-cli "$@")
    cd "$result"
  else
    waypoint-cli "$@"
  fi
}
alias waypoint=_waypoint__
```