### Build the Projects

First, clone this repository to your local machine:

``` sh
git clone https://github.com/eric9n/ncbi_dl.git
cd kun_peng
```

Ensure that both projects are built. You can do this by running the following command from the root of the workspace:

``` sh
cargo build --release
```

### Run the `ncbi` Example

Run the example script in the ncbi project to download the necessary files. Execute the following command from the root of the workspace:

``` sh
cargo run --release --example run_download --package ncbi_dl
```

This will run the run_download.rs example located in the ncbi project's examples directory. The script will:

1.  Ensure the necessary directories exist.
2.  Download the required files using the ncbi binary with the following commands:

-   ./target/release/ncbi_dl -d downloads gen -g archaea
-   ./target/release/ncbi_dl -d downloads tax

Example Output You should see output similar to the following:

``` txt
Executing command: /path/to/workspace/target/release/ncbi_dl -d /path/to/workspace/downloads gen -g archaea
NCBI binary output: [download output here]

Executing command: /path/to/workspace/target/release/ncbi_dl -d /path/to/workspace/downloads tax
NCBI binary output: [download output here]
```

The ncbi_dl binary is used to download resources from the NCBI website. Here is the help manual for the ncbi_dl binary:

``` sh
./target/release/ncbi_dl -h
ncbi_dl download resource

Usage: ncbi_dl [OPTIONS] <COMMAND>

Commands:
  taxonomy  Download taxonomy files from NCBI (alias: tax)
  genomes   Download genomes data from NCBI (alias: gen)
  help      Print this message or the help of the given subcommand(s)

Options:
  -d, --download-dir <DOWNLOAD_DIR>  Directory to store downloaded files [default: lib]
  -n, --num-threads <NUM_THREADS>    Number of threads to use for downloading [default: 20]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```
