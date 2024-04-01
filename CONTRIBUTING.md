## Development

---

### Prerequisities
In order to build and run the app locally, one needs to:
- Have [Rust](https://www.rust-lang.org/tools/install) installed
- Have some code editor or IDE, eg. [Zed](https://zed.dev/)

---

### Commands

#### Building

```
cargo build
```
> Initial build will take some time due to the need of pulling down all of the bevy dependencies.


#### Running
```
cargo run
```


#### Testing specific crate
```
cargo test -p {crate_name}
```


#### Crates inside the project
- `epub` (don't have unique name for crates.io yet)
- `ui` (wiring up bevy for user interaction)
