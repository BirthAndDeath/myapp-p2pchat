cd ./src-tauri/ && cargo cache --autoclean

yarn cache clean
cargo clean
# 安装官方缓存管理器
#cargo install cargo-cache --locked
#cargo install cargo-machete
#cargo install cargo-vet 
#cargo install cargo-crev
# 查看占用
#cargo cache --info
#cargo cache --autoclean