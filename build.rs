fn main() {
    glib_build_tools::compile_resources(
        &["templates/variant1"],
        "templates/variant1/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
}
