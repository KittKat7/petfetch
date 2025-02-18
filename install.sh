#!/bin/bash

# bash <(curl https://raw.githubusercontent.com/KittKat7/petfetch/main/install.sh)

TEXT="
KatBox Installer
-----------------
This will install files into ~/.local/opt/petfetch/ and will add a file in ~/.local/bin to start the program.

i: install/update
r: remove from system
c: cancel
"
echo "$TEXT"

read -p "Confirm? [I/r/c]: " CONFIRM
if [ -z $CONFIRM ] || [ $CONFIRM == "i" ];
then
	if [ ! -d ~/.local/bin ]; then mkdir -p ~/.local/bin; fi;
	
	if [ ! -d ~/.local/opt/petfetch ]; then
		mkdir -p ~/.local/opt/petfetch;
		git clone https://github.com/KittKat7/petfetch.git ~/.local/opt/petfetch;
	else
		cd ~/.local/opt/petfetch;
		git pull;
	fi;

	if [ ! -f ~/.local/bin/petfetch ]; then printf '#!/bin/bash\nbash ~/.local/opt/petfetch/petfetch.py $@' > ~/.local/bin/petfetch; fi;

	chmod +x ~/.local/bin/petfetch

	echo "COMPLETE"
elif [ $CONFIRM == "r" ]
then
	rm -fr ~/.local/opt/petfetch
	unlink ~/.local/bin/petfetch
fi


