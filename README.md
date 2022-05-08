# [Schema Serializer]() #

> *Please see [`LICENSE`](./LICENSE) for more information.*

## Table of Contents ##

[[_TOC_]]

## Overview ##

`CLI-Parser` is used to create, store, and update environment variable(s) used in both development and deployment
context. Additionally, through the simplicity of `key = value` variable assignments, `CLI-Parser` can construct
a variety of file-types:

- `*.env`
- `toml`
- `Yaml`
- `JSON`
- `ini`

### Design Philosophy ###

All environment variables are capable of being abstracted into a simple key-value structure generalizable to compiled,
runtime, & serialization languages:

```json
{
    "Key": "Value"
}
```

```yaml
---
Variable:
    Key: ...
    Value: ...
```

```c
struct Variable {
    char * key;
    char * value;
} Variable; 
```

```rust
struct Variable {
    key: &str,
    value: &str
}
```

```javascript
class Variable {
    static Key;
    static Value;
}
```

```python
class Variable:
    Key: str
    Value: str
```

Because of key-value serialization capabilities, `CLI-Parser` transforms command-line input into streamable HTTP requests; these requests can then interface distributed key-value systems such as

- AWS: **Secure Systems Manager**
- GitLab: **CI-CD**
- **etcd** (Open-Source)
- Hasicorp: **Vault**
- GitHub: **Actions**

## Usage ##

For Rust Installation, run the `Install.Bash` script.

In order to generate a new package, open a shell and issue the following:

```bash
cargo +nightly run --bin Parser -- --schema Schema.Yaml --type yaml

# cargo run --bin Serialize
# cargo run --bin Serialize -- --example "Test" --debug

# --> Nightly

# cargo +nightly run --bin Serialize
# cargo +nightly run --bin Serialize -- --example "Test" --debug
```

### Cargo ###

**Cargo**: *The Rust build tool and package manager.*

`rustup` ships with the latest, stable version of the Rust build tool and package manager,
also known as *Cargo*. Usage:

- **Install**: `cargo install --path "."`
  
- **Build Package**: `cargo build`
- **Run Program**: `cargo run`
- **Test Runtime**: `cargo test`
- **Documentation**: `cargo doc`
- **Publish Crate**: `cargo publish`

#### Nightly ####

- **Install**: `cargo install --path "."`
  
- **Build Package**: `cargo +nightly build`
- **Run Program**: `cargo +nightly run`
- **Test Runtime**: `cargo +nightly test`
- **Documentation**: `cargo +nightly doc`
- **Publish Crate**: `cargo +nightly publish`

To check the current version of `cargo`, run `cargo --version`

---

## Examples ##

```bash
cargo +nightly run --bin example-cli -- --help
```

---

## References ##

- [*The Rust Programming Language*](https://doc.rust-lang.org/book/title-page.html)
- [*Getting Started Guide*](https://www.rust-lang.org/learn/get-started)
- [*Installation*](https://www.rust-lang.org/tools/install)

- [*Server-API*](https://gitlab.cloud-technology.io/Infrastructure/Server-API.git)

---

[^1]: [Rust](https://doc.rust-lang.org/book/foreword.html)


```
cargo +nightly run --package Schema-Serialization --bin Schema-Serialization
```


## Git Module(s) ##

### YAML ###

```bash
git submodule add --force "https://github.com/dtolnay/serde-yaml.git" ./Modules/YAML
```

To remove the submodule,

```bash
git submodule deinit --force ./Modules/YAML
```

### JSON-Serialization ###

```bash
git submodule add --force "https://github.com/serde-rs/json.git" ./Modules/JSON
```

To remove the submodule,

```bash
git submodule deinit --force ./Modules/JSON
```

### JSON-Serialization Benchmarks ###

```bash
git submodule add --force "https://github.com/serde-rs/json-benchmark.git" ./Modules/JSON-Benchmarks
```

To remove the submodule, 

```bash
git submodule deinit --force ./Modules/JSON-Benchmarks
```

### Git-C ###

```bash
git submodule add --force "https://github.com/rust-lang/git2-rs.git" ./Modules/Git-C-Bindings
```

To remove the submodule,

```bash
git submodule deinit --force ./Modules/Git-C-Bindings
```