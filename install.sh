if [[ $(id -u) != 0 ]]; then
	echo "This script requires sudo privileges to copy files into /usr/local/bin, please"
	exit 1
fi

cargo build -r --bins

if [ $? -eq 0 ]; then
		echo "Compilation succesful"
else
		echo "[ERROR]: Compilation not succesful."
		exit 1
fi

files=("bin" "octal" "dec" "hex")

echo "Copy ${#files[@]} files into /usr/local/bin/ ? Y/n"
read input < /dev/tty

if [ -z "$input" ] || [ "$input" == "Y" ]; then
	for str in ${files[@]}; do
		cp "./target/debug/$str" "/usr/local/bin/"
	done
	echo "4 files copied into /usr/local/bin/"
else
	echo "No files were copied."
fi
