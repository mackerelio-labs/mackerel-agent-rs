# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "debian/buster64"
  config.vm.box_version = "10.3.0"
  # `vagrant plugin install vagrant-vbguest` を事前にしておくこと
  config.vm.synced_folder ".", "/src/"
  config.vm.hostname = "mackerelrs-test-#{ENV['USER']}"

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    sudo apt-get update
    sudo apt-get autoremove -y
    sudo apt install -y curl
    sudo apt install -y libssl-dev pkg-config build-essential

    cd /src
    if hash rustup; then
    rustup update
    else
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
    fi
    rustup component add rustfmt

    export PATH="$HOME/.cargo/bin:$HOME/.bin:$PATH"
    rustup component add rust-src
    cargo install cargo-watch

    echo "export PATH="$HOME/.cargo/bin:$PATH"; cd /src;  alias ll='ls -lG'" >> $HOME/.bashrc
  SHELL
end
