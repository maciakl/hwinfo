PROJ := "hwinfo"
VER := `sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml`

all: release

version:
    @echo "{{VER}}"

build:
    cargo build --release

hash: build
    sha256sum target/release/{{PROJ}}.zip > target/release/checksums-{{VER}}.txt
    cat target/release/checksums-{{VER}}.txt

release: hash
    git tag -a "v{{VER}}" -m "Release v{{VER}}"
    git push origin "v{{VER}}"
    gh release create "v{{VER}}" target/release/{{PROJ}}.zip target/release/checksums-{{VER}}.txt --title "v{{VER}}" --generate-notes

clean:
    cargo clean
