docker_available () {
    command -v docker >/dev/null 2>&1
}

install_web_ui () {
    # todo
    curl -L 
}

echo "🥞 Welcome to the pancake installer 🥞"

BRANCH=$(uname -m)
case $OSTYPE in 
    darwin*)
        if [[ "$BRANCH" != x86_64 && "$BRANCH" != "arm64" ]]; then
            echo "[!] Unsupported platform"
            read -r -n 1 -p "[!] Press any key to exit..."
            exit 1
        fi
        OS="darwin"
        ;;
    linux*)
        OS="linux"
        ;;
    *)
        echo "[!] Unsupported platform"
        read -r -n 1 -p "[!] Press any key to exit..."
        exit 1
        ;;
esac

echo "[*] Checking for docker..."
if ! docker_available; then
    echo -e "[!] Docker is not available in your PATH, \x1b[1mplease install it and rerun the installer\x1b[0m"
    read -r -n 1 -p "[!] Press any key to exit..."
    exit 1
fi

echo "[*] (sudo) Creating directory /usr/share/pancake"
sudo mkdir -p /usr/share/pancake

read -r -p "[?] Do you want to install the web ui? [Y/n]" answer
case $answer in 
    [Yy]*) 
    #todo