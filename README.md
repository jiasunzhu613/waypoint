# Waypoint
The modern alternative to pushd/popd built using Rust 🦀

## Basic usage
#### Waypoint stack
```
waypoint . # stores current directory as last remembers portable waypoint

cd testing
cd integration

waypoint back # restores current working directory as last placed portable waypoint
```

## Installation
1. **Install `waypointctl` from cargo:**
```
cargo install waypointctl
```

2. **Set up shell function**
> [!WARNING]
> Currently only supports bash. Support for other shells is in progress

```sh
# Add this to your .bashrc
function __waypoint__() {
  local result

  if [ "$1" = "back" ]; then
    result=$(waypointctl "$@")
    cd "$result"
  else
    waypointctl "$@"
  fi
}
alias waypoint=__waypoint__
```

## Roadmap
- [x] basic stack like pushd/popd (decide whether or not to mimic per-process design of pushd/popd)
- [x] list waypoints
- [ ] custom named waypoints
- [ ] build feature to allow for traversal without explicit placement of waypoints (automatic waypoints)