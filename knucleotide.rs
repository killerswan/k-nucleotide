import io::reader_util;

use std;
import std::map;
import std::map::hashmap;

fn main () {
   fn make_map() -> hashmap<str, uint> {
      ret map::hash_from_strs([]);
   }

   let frequencies =
      // k-mer size, frequency map, leftover string
      [( 1, make_map(), ""),
       ( 2, make_map(), ""),
       ( 3, make_map(), ""),
       ( 4, make_map(), ""),
       ( 6, make_map(), ""),
       (12, make_map(), ""),
       (18, make_map(), "")];

   let freqs = vec::len(frequencies);
   
   // update the frequency maps
   let update_freq = fn@(mm: hashmap<str, uint>, key: str) {

      alt mm.find(key) {
         option::none      { mm.insert(key, 1u      ); }
         option::some(val) { mm.insert(key, 1u + val); }
      }
   };


   // FIXME: wtf?
   let update_frequencies = fn@(line: str) {
      io::println("GEARS SPINNING...");
      /*
      let len = vec::len(frequencies);
      for frequencies.each {|freq|
         let (sz, mm, carry) = freq;
         update_freq(mm, "one");
      }
      */
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
               update_frequencies(line);
         }

         // whatever
         _ { }
      }
   }

   //io::println(#fmt["one: %u", frequencies.get("one")]);
   
   let (_, m, _) = frequencies[0];
   io::println(#fmt["one: %u", m.get("one")]);
}


