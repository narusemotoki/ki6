FROM ubuntu:21.04

ENV DEBIAN_FRONTEND noninteractive
ENV PATH $PATH:/home/ubuntu/.cargo/bin

RUN useradd --create-home --shell /bin/bash --uid 1000 ubuntu

# --no-install-recommendsを与えるとelectronが"electron exited with signal SIGTRAP"というエラーを出し起動できなくなります。何を足せば良いか特定できたら--no-install-recommendsを与えます。
RUN apt update && apt install --assume-yes libxshmfence1 libnss3 libatk1.0-0 libatk-bridge2.0-0 libdrm2 libgtk-3-0 libgbm-dev libasound2 libcanberra-gtk3-0 libgtkextra-dev libgconf2-dev libxtst-dev libxss1 nodejs npm fonts-noto-cjk fonts-noto-cjk-extra fonts-noto-color-emoji curl

USER ubuntu

# WAMSをターゲットにしたビルドはRustupでインスールしたRustcを使わないと、ターゲットを手動でインストールしないといけません。手間なのでaptを使わずにRustcをインストールします。
# shの-sとrustupの-yには短縮形しかありません。
RUN curl --proto '=https' --tlsv1.2 --silent --show-error https://sh.rustup.rs | sh -s -- -y
RUN rustup install stable
RUN cargo install cargo-make
