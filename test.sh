for file in $1/**.t
do
	echo $file
	cargo run -q -- "$file" $2
	echo "
	"
done
