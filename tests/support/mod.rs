use std::path::Path;

use st::vm::VirtualMachine;

pub fn vm_with_sources(sources: &[(&str, &str)]) -> VirtualMachine {
    let vm = VirtualMachine::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("smalltalk");
    for (package, file) in sources {
        let path = root.join(package).join(file);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("Failed to read {}: {error}", path.display()));
        vm.compile_src(&source)
            .unwrap_or_else(|error| panic!("Failed to load {}: {error}", path.display()));
    }
    vm
}
