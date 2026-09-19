use markusunkel_com::content::Content;

fn main() {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("preprocess") => {
            let started = std::time::Instant::now();
            if let Err(error) = Content::load() {
                eprintln!("content: {error}");
                std::process::exit(1);
            }
            println!("content loaded in {:.1?}", started.elapsed());
        }
        Some(task) => {
            eprintln!("unknown task: {task}");
            std::process::exit(1);
        }
        None => {
            std::process::exit(1);
        }
    }
}
