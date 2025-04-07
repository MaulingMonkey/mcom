@pushd "%~dp0.." && setlocal

@set CARGO_TARGET_DIR=target/integration
@set RUSTUP_TOOLCHAIN=



cargo build --no-default-features                                                           || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-2000         || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-xp           || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-vista        || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-7            || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-8            || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-8-1          || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-10           || goto :err
cargo test  --no-default-features --features winapi-family-desktop-app,windows-latest       || goto :err

cargo check --no-default-features                                                           || goto :err
cargo check --no-default-features --features winapi-family-server,windows-2000              || goto :err
cargo check --no-default-features --features winapi-family-server,windows-xp                || goto :err
cargo check --no-default-features --features winapi-family-server,windows-vista             || goto :err
cargo check --no-default-features --features winapi-family-server,windows-7                 || goto :err
cargo check --no-default-features --features winapi-family-server,windows-8                 || goto :err
cargo check --no-default-features --features winapi-family-server,windows-8-1               || goto :err
cargo check --no-default-features --features winapi-family-server,windows-10                || goto :err
cargo check --no-default-features --features winapi-family-server,windows-latest            || goto :err

cargo check --no-default-features                                                           || goto :err
cargo check --no-default-features --features winapi-family-system,windows-2000              || goto :err
cargo check --no-default-features --features winapi-family-system,windows-xp                || goto :err
cargo check --no-default-features --features winapi-family-system,windows-vista             || goto :err
cargo check --no-default-features --features winapi-family-system,windows-7                 || goto :err
cargo check --no-default-features --features winapi-family-system,windows-8                 || goto :err
cargo check --no-default-features --features winapi-family-system,windows-8-1               || goto :err
cargo check --no-default-features --features winapi-family-system,windows-10                || goto :err
cargo check --no-default-features --features winapi-family-system,windows-latest            || goto :err

cargo check --no-default-features                                                           || goto :err
cargo check --no-default-features --features winapi-family-pc-app,windows-8                 || goto :err
cargo check --no-default-features --features winapi-family-pc-app,windows-8-1               || goto :err
cargo check --no-default-features --features winapi-family-pc-app,windows-10                || goto :err
cargo check --no-default-features --features winapi-family-pc-app,windows-latest            || goto :err

cargo check --no-default-features                                                           || goto :err
cargo check --no-default-features --features winapi-family-phone-app,windows-8              || goto :err
cargo check --no-default-features --features winapi-family-phone-app,windows-8-1            || goto :err
cargo check --no-default-features --features winapi-family-phone-app,windows-10             || goto :err
cargo check --no-default-features --features winapi-family-phone-app,windows-latest         || goto :err

cargo check --no-default-features                                                           || goto :err
cargo check --no-default-features --features winapi-family-games,windows-8                  || goto :err
cargo check --no-default-features --features winapi-family-games,windows-8-1                || goto :err
cargo check --no-default-features --features winapi-family-games,windows-10                 || goto :err
cargo check --no-default-features --features winapi-family-games,windows-latest             || goto :err



cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-2000   || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-xp     || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-vista  || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-7      || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-8      || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-8-1    || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-10     || goto :err
cargo check --no-default-features --features alloc,winapi-family-desktop-app,windows-latest || goto :err

cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-2000        || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-xp          || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-vista       || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-7           || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-8           || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-8-1         || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-10          || goto :err
cargo check --no-default-features --features alloc,winapi-family-server,windows-latest      || goto :err

cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-2000        || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-xp          || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-vista       || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-7           || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-8           || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-8-1         || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-10          || goto :err
cargo check --no-default-features --features alloc,winapi-family-system,windows-latest      || goto :err

cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-pc-app,windows-8           || goto :err
cargo check --no-default-features --features alloc,winapi-family-pc-app,windows-8-1         || goto :err
cargo check --no-default-features --features alloc,winapi-family-pc-app,windows-10          || goto :err
cargo check --no-default-features --features alloc,winapi-family-pc-app,windows-latest      || goto :err

cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-phone-app,windows-8        || goto :err
cargo check --no-default-features --features alloc,winapi-family-phone-app,windows-8-1      || goto :err
cargo check --no-default-features --features alloc,winapi-family-phone-app,windows-10       || goto :err
cargo check --no-default-features --features alloc,winapi-family-phone-app,windows-latest   || goto :err

cargo check --no-default-features --features alloc                                          || goto :err
cargo check --no-default-features --features alloc,winapi-family-games,windows-8            || goto :err
cargo check --no-default-features --features alloc,winapi-family-games,windows-8-1          || goto :err
cargo check --no-default-features --features alloc,winapi-family-games,windows-10           || goto :err
cargo check --no-default-features --features alloc,winapi-family-games,windows-latest       || goto :err



cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-2000     || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-xp       || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-vista    || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-7        || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-8        || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-8-1      || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-10       || goto :err
cargo check --no-default-features --features std,winapi-family-desktop-app,windows-latest   || goto :err

cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-2000          || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-xp            || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-vista         || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-7             || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-8             || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-8-1           || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-10            || goto :err
cargo check --no-default-features --features std,winapi-family-server,windows-latest        || goto :err

cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-2000          || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-xp            || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-vista         || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-7             || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-8             || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-8-1           || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-10            || goto :err
cargo check --no-default-features --features std,winapi-family-system,windows-latest        || goto :err

cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-pc-app,windows-8             || goto :err
cargo check --no-default-features --features std,winapi-family-pc-app,windows-8-1           || goto :err
cargo check --no-default-features --features std,winapi-family-pc-app,windows-10            || goto :err
cargo check --no-default-features --features std,winapi-family-pc-app,windows-latest        || goto :err

cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-phone-app,windows-8          || goto :err
cargo check --no-default-features --features std,winapi-family-phone-app,windows-8-1        || goto :err
cargo check --no-default-features --features std,winapi-family-phone-app,windows-10         || goto :err
cargo check --no-default-features --features std,winapi-family-phone-app,windows-latest     || goto :err

cargo check --no-default-features --features std                                            || goto :err
cargo check --no-default-features --features std,winapi-family-games,windows-8              || goto :err
cargo check --no-default-features --features std,winapi-family-games,windows-8-1            || goto :err
cargo check --no-default-features --features std,winapi-family-games,windows-10             || goto :err
cargo check --no-default-features --features std,winapi-family-games,windows-latest         || goto :err



:err
@popd && endlocal && exit /b %ERRORLEVEL%
