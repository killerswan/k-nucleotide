rustc -g knucleotide.rs
valgrind --tool=callgrind ./knucleotide < knucleotide-input.txt
kcachegrind callgrind*
