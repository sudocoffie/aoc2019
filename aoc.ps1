param([String]$name="dayoneone")

cargo new $name

rm $name/src/main.rs
cp ./main-template.rs $name/src/main.rs
New-Item $name/input.txt
code $name/.
cd $name
