cargo test --package nilsmal-chess --lib -- tests::test_promotion --exact --nocapture | sed -n '/running 1 test/,/test tests::run_main_function ... ok/p'

