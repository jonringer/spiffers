use std::io::Error;

fn main() -> Result<(), Error> {
    tonic_build::compile_protos("protos/spire/common/common.proto")
}
