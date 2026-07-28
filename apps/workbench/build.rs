fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=styles.css");
    println!("cargo:rerun-if-changed=workbench.css");

    topcoat::tailwind::BuildConfig::new()
        .input("workbench.css")
        .render()
        .expect("the workbench Tailwind stylesheet builds");

    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("feather")
        .stage()
        .expect("the workbench icon set stages");
}
