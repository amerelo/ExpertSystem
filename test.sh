for file in ./**/**/**.t
do
	echo $file
	cargo run -q -- "$file"
done
