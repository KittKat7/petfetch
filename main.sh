#!/bin/bash
path="$(dirname "$0")"
pet="cat"

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
info+=("$distro $distrov")
info+=("$system $arch")
info+=("$version")
info+=("$cpu")

pad="  "

len=0;
i=0;
while IFS= read -r line; do
	if [ ${#line} -ge $len ]
	then
		len=${#line}
	fi
	logo[$i]=$line
	i=$i+1
done < "$path/$pet.ascii"

len=$(expr $len + ${#pad})

j=0
for i in "${logo[@]}"
do
	printf "%-${len}s%s\n" "$pad$i" "$pad: ${info[j]}"
	j=$j+1
done






