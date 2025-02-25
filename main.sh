#!/bin/bash
path="$(dirname "$0")"
pet="cat"
color=""

while test $# -gt 0; do
	case "$1" in
		-h|--help)
			echo "help message"
			exit 0
			;;
		-m|--modify)
			bash $path/install.sh
			exit 0
			;;
		-p|--pet*)
			shift
			pet=`echo $1 | sed -e 's/^[^=]*=//g'`
			shift
			;;
		-c|--color*)
			shift
			color=`echo $1 | sed -e 's/^[^=]*=//g'`
			shift
			;;
		*)
			break
			;;
	esac
done


title="PetFetch - $pet"

system=$(uname);
arch=$(uname -m);
version=$(uname -r);
cpu=$(lscpu | grep 'Model name' | cut -f 2 -d ":" | awk '{$1=$1}1');
distro="$(awk -F= '/^NAME=/ {gsub(/"/, "", $2); print $2}' /etc/os-release)"
distrov="$(awk -F= '/^VERSION_ID=/ {gsub(/"/, "", $2); print $2}' /etc/os-release)"

info[0]=$title
info+=("$distro $distrov $arch")
#info+=("$system $arch")
info+=("$version")
info+=("$cpu")

pad="  "

cyan="\e[1;5;36m"
pink="\e[1;5;35m"
white="\e[1;37m"

tra=("$cyan" "$pink" "$white" "$pink" "$cyan")
reset="\e[0m"

colors=()

if [[ "$color" == "tra" ]];
then
	colors=("${tra[@]}")
fi

if [[ "$pet" == "blahaj" ]];
then
	pet="shark"
	colors=("${tra[@]}")
fi

len=0;
i=0;
while IFS="" read -r line || [ -n "$line" ]; do
	if [ ${#line} -ge $len ]
	then
		len=${#line}
	fi
	logo[$i]=$line
	i=$i+1
done < "$path/$pet.ascii"

len=$(expr $len + ${#pad})

printf "$reset\n"
j=0
for i in "${logo[@]}"
do
	c=""
	if [ ${#colors} -gt 0 ]
	then
		cl=${#colors[@]}
		ll=${#logo[@]}

		x=$(( ll / $cl ))
		if [ $x -eq 0 ]
		then
			x=$(( x + 1 ))
		fi
		x=$(( j / $x  ))
		c=${colors[$x]}
	fi
	printf "$c%-${len}s$reset%s\n" "$pad$i" "$pad: ${info[j]}"
	j=$(( j + 1 ))
done
printf "$reset\n"






