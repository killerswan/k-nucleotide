
import io::reader_util;

fn main () {
   // define leftovers
   // define maps

   let update_kmer_freqs = fn@(line: str) {
      // update the frequency maps
      io::println("GEARS SPINNING...");
   };

   let mut proc_mode = false;

   let rdr = io::stdin();

   while !rdr.eof() {
      let line: str = rdr.read_line();
      if str::len(line) == 0u { cont; }
      alt (line[0], proc_mode) {

         // start processing if this is the one
         ('>' as u8, false) {
            alt str::find_str_from(line, "THREE", 1u) {
               option::some(_) { proc_mode = true; }
               option::none { }
            }
         }

         // break our processing
         ('>' as u8, true) {
            ret;
         }

         // process the sequence for k-mers
         (_, true) {
               update_kmer_freqs(line);
         }

         // whatever
         _ { }
      }
   }
}


