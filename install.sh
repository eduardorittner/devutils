cargo build --release
cargo install --path ./

if [[ ":$PATH:" == *":$HOME/.cargo/bin"* ]]; then
		echo "Binaries installed."
else
		echo "It looks like '~/.cargo/bin/' is not in path, would you like to add it? Y/n"
		read add_to_path

		if [[ -z $add_to_path || "$add_to_path" == "y" || "$add_to_path" == "Y" ]]; then
				echo "export PATH='~/.cargo.bin:$PATH'" >> ~/.bashrc
				echo "~/.cargo/bin/ added to path, run 'source ~/.bashrc' for changes to take effect."
		else
				echo "~/.cargo/bin/ not added to path."
		fi
fi
