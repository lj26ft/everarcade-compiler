use std::fs;
#[test]
fn civilization_genesis_vectors_exist_and_non_empty() {
 let base = "tests/test_vectors/civilization_genesis";
 for name in ["genesis","domain","constitution","treasury","fiscal","monetary","asset","receipt","replay","proof","checkpoint","sync"] {
   let bytes = fs::read(format!("{base}/{name}.bin")).expect("vector must exist");
   assert!(!bytes.is_empty(), "vector {name} should be non-empty");
 }
}
