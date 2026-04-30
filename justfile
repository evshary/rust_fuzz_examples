clean:
    rm -rf afl_rs_example/out \
           afl_rs_example/target
    rm -rf cargo_fuzzy_example/target
    rm -rf honggfuzz_rs_example/hfuzz_target \
           honggfuzz_rs_example/target \
           honggfuzz_rs_example/hfuzz_workspace/parse_port/*.TXT \
           honggfuzz_rs_example/hfuzz_workspace/parse_port/*.fuzz \
           honggfuzz_rs_example/hfuzz_workspace/parse_port/input/*.cov
    rm -rf proptest_example/proptest-regressions \
           proptest_example/target
    rm -rf arbitrary_example/target
    rm -rf libafl_example/target
    rm -rf bolero_example/target
    rm -rf loom_example/target
    rm -rf target
