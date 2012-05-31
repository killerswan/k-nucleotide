
import io::reader_util;

fn main () {
   let rdr = io::stdin();
   seek_matching_sequence(rdr);
}

fn seek_matching_sequence(rdr: io::reader) {
   let mut proc_mode = false;

   fn update_kmer_freqs(line: str) {
      io::println("GEARS SPINNING...");
   }

   while !rdr.eof() {
      let line: str = rdr.read_line();
      if str::len(line) == 0u { cont; }
      alt (line[0], proc_mode) {
         ('>' as u8, false) {
            alt str::find_str_from(line, "THREE", 1u) {
               option::some(_) { proc_mode = true; }
               option::none    { }
            }
         }
         ('>' as u8, true) {
            ret;
         }
         (_, true) {
               update_kmer_freqs(line);
         }
         _ { }
      }
   }
}


