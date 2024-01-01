use deko::CONFIG;

fn main()
{
  println!("➡️ starting deko_cli...");
  let config = CONFIG
    .lock()
    .expect("failed to get config");
  println!("☑️ deko_cli finished running!");
}