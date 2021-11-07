# How to run model

To compile and run the model, you will need to have Rust and Cargo installed on your system.
You can find out how to do that [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Our system runs using configuration files. A default file, along with some other sample configurations can be found in the `Commons/config` folder.

## On Linux

```bash
cd Commons
cargo build --release
./target/release/commons -h # see help
```
After running this, you can run our final configuration (or any other config):
```bash
./target/release/commons <out_directory> -c ./config/final_run.toml
```

## On Windows

```powershell
cd Commons
cargo build --release
.\target\release\commons.exe -h # see help
```
After running this, you can run our final configuration (or any other config):
```bash
./target/release/commons.exe .\data\final_run -c .\config\final_run.toml
```
