fn main() {
    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .expect("the workbench Tailwind stylesheet builds");
}
