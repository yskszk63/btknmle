fn main() {
    cc::Build::new().file("src/native.c").compile("native");
}
