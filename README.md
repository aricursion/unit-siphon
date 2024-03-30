# Unit Siphon
A tool to extract unit clauses in real time from generated DRAT proofs

## Building
Simply run `cargo build` to build the tool and a binary will be at
`/target/debug/unit_siphon/`. Alternatively, use `cargo run`.

## Usage
Simply pass the name of the cnf file you would like to extract units from such as
`cargo run -- -c myCnf.cnf`. By default it will simply print all the units to `stdout`
in real time.
