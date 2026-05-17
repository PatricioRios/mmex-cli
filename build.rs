fn main() {
    println!("cargo:warning=------------------------------------------------------------------");
    println!("cargo:warning=MMEX CLI installed successfully!");
    println!("cargo:warning=");
    println!("cargo:warning=To enable shell autocompletion, run the following commands:");
    println!("cargo:warning=");
    println!("cargo:warning=[Bash]");
    println!("cargo:warning=  mkdir -p ~/.local/share/bash-completion/completions");
    println!("cargo:warning=  mmex completions bash > ~/.local/share/bash-completion/completions/mmex");
    println!("cargo:warning=");
    println!("cargo:warning=[Zsh]");
    println!("cargo:warning=  mkdir -p ~/.zfunc");
    println!("cargo:warning=  mmex completions zsh > ~/.zfunc/_mmex");
    println!("cargo:warning=  # Ensure fpath+=~/.zfunc is in your ~/.zshrc before compinit");
    println!("cargo:warning=");
    println!("cargo:warning=[Fish]");
    println!("cargo:warning=  mmex completions fish > ~/.config/fish/completions/mmex.fish");
    println!("cargo:warning=------------------------------------------------------------------");
}
