fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=styles.css");
    println!("cargo:rerun-if-changed=workbench.css");

    topcoat::tailwind::BuildConfig::new()
        .input("workbench.css")
        .render()
        .expect("the workbench Tailwind stylesheet builds");
}
