use chronicles_io::ChronicleMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<ChronicleMetadata>();
}