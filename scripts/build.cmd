@pushd "%~dp0.." && setlocal

cargo fetch                                                             || goto :err
cargo check --frozen --all-targets --all-features                       || goto :err
cargo test  --frozen                                                    || goto :err
cargo build --frozen --all-targets --all-features                       || goto :err
cargo +nightly doc --frozen --no-deps || cargo doc --frozen --no-deps   || goto :err
@call scripts\integration.cmd                                           || goto :err

:err
@popd && endlocal && exit /b %ERRORLEVEL%
