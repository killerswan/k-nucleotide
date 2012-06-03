#time ./knucleotide < knucleotide-input.txt
#time diff -u knucleotide-output.txt <(./knucleotide < knucleotide-input.txt)
time diff -u knucleotide-output.txt <(./knucleo-multi < knucleotide-input.txt)
