curl -O http://shootout.alioth.debian.org/download/knucleotide-input.txt
curl -O http://shootout.alioth.debian.org/download/knucleotide-output.txt

rustc -O knucleotide.rs

time ./knucleotide < knucleotide-input.txt
# time diff -u knucleotide-output.txt <(./knucleotide < knucleotide-input.txt)
