import io::reader_util;

use std;
import std::map;
import std::map::hashmap;

fn main () {
   // define leftovers
   // define maps

   let m: hashmap<str, uint> = map::hash_from_strs([]);
   

   let update_kmer_freqs = fn@(line: str) {
      // update the frequency maps

      alt m.find("one") {
         option::none      { m.insert("one", 1u      ); }
         option::some(val) { m.insert("one", 1u + val); }
      }
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
            break;
         }

         // process the sequence for k-mers
         (_, true) {
               update_kmer_freqs(line);
         }

         // whatever
         _ { }
      }
   }

   io::println(#fmt["one: %u", m.get("one")]);
}


