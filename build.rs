use std::process::Command;

fn main() {
    // Compile GSettings schemas
    let status = Command::new("glib-compile-schemas")
        .arg("data") // Folder containing *.gschema.xml
        .status()
        .expect("Failed to compile GSettings schemas");

    if !status.success() {
        panic!("Failed to compile schemas: {:?}", status);
    }

    // Compile GtkBuilder resources
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
}
