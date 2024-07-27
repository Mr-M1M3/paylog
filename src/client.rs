pub struct Client<'a>{
pub name: &'a str,
   log: Log<'a>
}
#[derive(Debug)]
pub struct Log<'a>{
   entries: Vec<LogEntry<'a>>
}
#[derive(Debug)]
pub struct LogEntry<'a>{
   date: &'a str,
   amount: isize
}

impl<'a> Client<'a>{
   pub fn new(){
      todo!();
   }
   pub fn display(){
      todo!();
   }
   pub fn export(){
      todo!();
   }
}

impl<'a> Log<'a>{
   pub fn from_str(to_parse: &'a str) -> Self{
       let entries = to_parse
			.split(";")
			.map(|entry| {
			   let e = entry
				   .split(":")
				   .collect::<Vec<_>>();
			   if e.len() < 2 {
			      panic!("corrupted data");
			   }
			   LogEntry::new(e[0],
					 e[1]
					 .trim()
					 .parse::<isize>()
					 .expect("corrupted data"))
			})
			.collect::<Vec<_>>();
       Self {
	    entries
       }
   }
   pub fn calculate(&self) -> isize{
      let mut acc: isize = 0;
      self.entries
		.iter()
		.for_each(|entry| acc += entry.amount);
      acc
   }
   pub fn push(&mut self, date: &'a str, amount: isize){
      self.entries.push(LogEntry::new(date, amount));
   }
}
impl<'a> LogEntry<'a>{
   pub fn new(date: &'a str, amount: isize) -> Self{
      Self {
           date,
           amount
      }
   }
}
#[cfg(test)]
mod test {
use super::*;
//   println!("{:#?}", log);
#[test]
fn calc_total_amount(){
   let mut log: Log = Log::from_str("20.5.2024: +500");
      log.push("30-7-24", -41);
   assert_eq!(log.calculate(), 459);
}
}
