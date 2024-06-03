if [[ $(id -u) != 0 ]]; then
	echo "This script requires sudo privileges to copy files into /usr/local/bin, please"
	exit 1
fi

files=("bin" "octal" "dec" "hex")

echo "Delete ${#files[@]} files from /usr/local/bin/ ? Y/n"
for str in ${files[@]}; do
	echo "/usr/local/bin/$str"
done
read input < /dev/tty

if [ -z "$input" ] || [ "$input" == "Y" ]; then
	for str in ${files[@]}; do
		rm "/usr/local/bin/$str"
	done
	echo "4 files deleted from /usr/local/bin/"
else
	echo "No files were deleted."
fi

