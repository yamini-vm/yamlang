./target/debug/yamlang $1.ym
yamasm $1.yas
yamini a.out
rm a.out
rm $1.yas