FROM ubuntu:25.10

# Ensure all packages are updated to the latest security patches
RUN apt-get update && apt-get upgrade -y && apt-get install -y  --no-install-recommends\
  bash \
  black \
  build-essential \
  clang \
  clang-format \
  clang-tidy \
  clang-tools \
  cmake \
  curl \
  flake8 \
  gcc \
  gcc-aarch64-linux-gnu \
  gdb \
  git \
  imagemagick \
  jq \
  libc6-dev-arm64-cross \
  libgtest-dev \
  libssl-dev \
  linux-libc-dev-arm64-cross \
  lldb \
  lsof \
  man \
  moreutils \
  net-tools \
  openssh-client \
  pkg-config \
  procps \
  python3 \
  python3-pip \
  sudo \
  vim \
  wget


# Install Rust
ENV RUST_VERSION=1.92.0
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
RUN curl --fail https://sh.rustup.rs -sSf \
  | sh -s -- -y --default-toolchain=${RUST_VERSION} -t x86_64-unknown-linux-musl -t aarch64-unknown-linux-gnu
ENV PATH=${CARGO_HOME}/bin:${PATH}

# Add missing components for rust
RUN rustup component add rustfmt clippy

# Install python sql web browser
# https://github.com/coleifer/sqlite-web/tree/master
RUN pip install sqlite-web --break-system-packages


# Add scripts
ENV PECUNIA_SCRIPT_DIR=/opt/pecunia/scripts

ADD ./build-env/scripts ${PECUNIA_SCRIPT_DIR}
ADD ./build-env/.githooks /.githooks


ENV PECUNIA_SRC="/workspace"

RUN cp ${PECUNIA_SCRIPT_DIR}/pecunia.sh /usr/local/bin/pecunia
RUN cp ${PECUNIA_SCRIPT_DIR}/pecunia-web.sh /usr/local/bin/pecunia-web

RUN chmod +x /usr/local/bin/pecunia*


RUN useradd -m -s /bin/bash cylon &&\
  usermod -aG sudo cylon &&\
  passwd -d cylon &&\
  echo "cylon ALL=(ALL) NOPASSWD:ALL" | tee /etc/sudoers.d/cylon

# Final cleanup
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /var/log/* /usr/share/local/*
RUN userdel -fr ubuntu || true

# Final permission fix
RUN sudo chmod -R 0777 /opt/