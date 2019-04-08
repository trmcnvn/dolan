workflow "CI" {
  on = "push"
  resolves = ["Cargo"]
}

action "Cargo" {
  uses = "icepuma/rust-action@1.0.4"
}
