The native backend is egui_glow (using glow) and should work out-of-the-box on Mac and Windows, but on Linux you need to first run:

sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev

On Fedora Rawhide you need to run:

dnf install clang clang-devel clang-tools-extra speech-dispatcher-devel libxkbcommon-devel pkg-config openssl-devel libxcb-devel