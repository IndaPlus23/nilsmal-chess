cargo test --package nilsmal-chess --lib -- tests::run_main_function --exact --nocapture | sed -n '/running 1 test/,/test tests::run_main_function ... ok/p'

