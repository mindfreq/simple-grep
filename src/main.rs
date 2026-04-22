
fn main() {
    let args: Vec<String> = std::env::args().collect();
    m_grep::Config::build(args);
}
