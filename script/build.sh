# !/bin/bash
rustup target add x86_64-unknown-linux-musl
brew install FiloSottile/musl-cross/musl-cross
mkdir ../.cargo
touch ../.cargo/config
echo "[target.x86_64-unknown-linux-musl]
      linker = \"x86_64-linux-musl-gcc\"
      " > ../.cargo/config
CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl
##构建 Docker 镜像