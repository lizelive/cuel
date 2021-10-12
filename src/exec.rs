pub struct Exec {
    entrypoint: String,
    args: Vec<String>,
    env: Option<Vec<(String, String)>>,
    cwd: Option<String>,
}