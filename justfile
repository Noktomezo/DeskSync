# Список всех команд по умолчанию
default:
    @just --summary

# Сборка проекта в режиме отладки
build:
    cargo build

# Сборка проекта в режиме релиза с последующим UPX сжатием
release:
    cargo build --release
    upx --best --lzma target/release/DeskSync.exe
