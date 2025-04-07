@pushd "%~dp0.." && setlocal

set RUSTUP_TOOLCHAIN=
cargo generate-lockfile                                                                     || goto :err
cargo fetch                                                                                 || goto :err

set RUSTUP_TOOLCHAIN=stable
cargo fetch                                                                                 || goto :err

set RUSTUP_TOOLCHAIN=nightly
cargo fetch                                                                                 || goto :err



@set CARGO_TARGET_DIR=target/msrv-all-features
@set RUSTUP_TOOLCHAIN=
cargo check --frozen --all-targets --all-features                                           || goto :err
cargo test  --frozen               --all-features                                           || goto :err
cargo build --frozen --all-targets --all-features                                           || goto :err

@set CARGO_TARGET_DIR=target/msrv-no-std
@set RUSTUP_TOOLCHAIN=
cargo check --frozen --all-targets --no-default-features                                    || goto :err
cargo test  --frozen               --no-default-features                                    || goto :err
cargo build --frozen --all-targets --no-default-features                                    || goto :err

@set CARGO_TARGET_DIR=target/stable-all-features
@set RUSTUP_TOOLCHAIN=stable
cargo check --frozen --all-targets --all-features                                           || goto :err
cargo test  --frozen               --all-features                                           || goto :err
cargo build --frozen --all-targets --all-features                                           || goto :err

@set CARGO_TARGET_DIR=target/nightly-all-features
@set RUSTUP_TOOLCHAIN=nightly
cargo check --frozen --all-targets --all-features                                           || goto :err
cargo test  --frozen               --all-features                                           || goto :err
cargo build --frozen --all-targets --all-features                                           || goto :err
cargo doc --frozen --no-deps --all-features || cargo doc --frozen --no-deps --all-features  || goto :err



@call scripts\integration.cmd                                                               || goto :err



:err
@popd && endlocal && exit /b %ERRORLEVEL%
