const USAGE: &str = "
Image Viewer.

Usage:
  image_viewer.exe <path>
";
 
#[derive(RustcDecodable)]
pub struct Args {
    pub arg_path: String
}

pub fn read_command_line() -> Args {
    use docopt::Docopt;
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    args
}
