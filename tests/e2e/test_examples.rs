use markdown_to_html::convert;
use std::fs;
use std::path::Path;

#[test]
fn test_all_examples() {
    let examples_dir = Path::new("_site/examples");
    let mut tested = 0;
    let mut failures = Vec::new();

    for entry in fs::read_dir(examples_dir).expect("_site/examples/ dir must exist") {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(true, |e| e != "md") {
            continue;
        }
        let stem = path.file_stem().unwrap().to_string_lossy().to_string();

        let expect_path = examples_dir.join(format!("{}.expect.html", stem));
        if !expect_path.exists() {
            continue;
        }

        let input = fs::read_to_string(&path).unwrap();
        let expected = fs::read_to_string(&expect_path).unwrap();
        let result = convert(&input);

        if result.trim() != expected.trim() {
            eprintln!("=== MISMATCH: {} ===", stem);
            eprintln!("--- expected ---\n{}", expected.trim());
            eprintln!("--- got ---\n{}", result.trim());
            failures.push(stem);
        }
        tested += 1;
    }

    assert!(tested > 0, "no examples found to test");
    assert!(
        failures.is_empty(),
        "html output mismatch for: {}",
        failures.join(", ")
    );
}
