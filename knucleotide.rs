
fn main () {
   import io::reader_util;

   io::println("Hello.");

   let rdr: io::reader = io::stdin();
   while !rdr.eof() {
      let ln: str = rdr.read_line();
      io::println("line: " + ln);
   }
}

