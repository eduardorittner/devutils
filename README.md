# DevUtils

Simple binary utilities suite for software developers.

Currently comprised of only numeric base visualization and conversion.

## Installation

After cloning the repo, simply run sudo ./install.sh which will compile the code and copy the binaries to /usr/local/bin/. If desired, it's possible to just compile the code yourself using cargo and use the generated binaries in /target/debug/ as you see fit.
There is an uninstall.sh which will delete the binaries from /usr/local/bin/. You can alsodo that yourself, if for any reason necessary. No further action is needed other than just directly deleting them.

## Numeric Bases

Support for displaying numbers in binary, octal, decimal an hexadecimal base with
the bin, octal, dec and hex commands, respectively.
The binaries are separate for ease of use, all of them accept a number in any base and format, but each only outputs the number in their given base.
Some accepted number formats are:
10
0xFF
0xff
0b01
0o10
-20
0001
+31
+0b10

