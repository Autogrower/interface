fn main() {
    glib_build_tools::compile_resources(
        &["src/templates/variant1"],
        "src/templates/variant1/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
}
