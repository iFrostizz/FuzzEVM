mod abi;

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let artifacts_path = args.next().unwrap();
    let root = env::current_dir().unwrap();
    let artifacts = root.join(artifacts_path);

    let abis = abi::abis(artifacts);
    let first = abis.first().unwrap();
    for function in first.functions() {
        let input = function.encode_input(tokens)
    }
}
