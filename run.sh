#time ./knucleotide < knucleotide-input.txt
time diff -u knucleotide-output.txt <(./shootout-k-nucleotide < knucleotide-input.txt)
