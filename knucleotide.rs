
import io::reader_util;

fn main () {

   io::println("Hello.");

   let rdr = io::stdin();

   seek_matching_sequence(rdr);
}

fn seek_matching_sequence(rdr: io::reader) {
   while !rdr.eof() {
      let ln: str = rdr.read_line();
      alt ln[0] {
         '>' as u8 { alt str::find_str_from(ln, "THREE", 1u) {
                  option::some(_) { process_sequence(rdr) }
                  option::none    { }
             }}
         _   { }
      }
   }
}

fn process_sequence(rdr: io::reader) {
   io::println("processing...");
}

