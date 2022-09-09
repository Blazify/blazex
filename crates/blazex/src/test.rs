#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn compile() {
        let mut test_dir = std::env::current_dir().unwrap();
        test_dir.pop();
        test_dir.pop();
        test_dir.push("tests");
        let parent_folder = std::fs::read_dir(test_dir).unwrap();
        for folder in parent_folder {
            if folder.is_err() {
                continue;
            }

            let child_folder = std::fs::read_dir(folder.unwrap().path());
            if child_folder.is_err() {
                continue;
            }

            for file in child_folder.unwrap() {
                if file.is_err() {
                    continue;
                }

                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                if !file_name.ends_with(".bzx") {
                    continue;
                }

                let file_path = file.path().into_os_string().into_string().unwrap();
                let out_file = file_path.replace(".bzx", ".o");
                let cnt = std::fs::read_to_string(file_path).unwrap();
                unsafe {
                    let res = super::super::compile(file_name, cnt, false, false, out_file, false);
                    assert_eq!(res, 0);
                }
            }
        }
    }

    #[test]
    fn run() {
        let mut test_dir = std::env::current_dir().unwrap();
        test_dir.pop();
        test_dir.pop();
        test_dir.push("tests");
        let parent_folder = std::fs::read_dir(test_dir).unwrap();
        for folder in parent_folder {
            if folder.is_err() {
                continue;
            }

            let child_folder = std::fs::read_dir(folder.unwrap().path());
            if child_folder.is_err() {
                continue;
            }

            for file in child_folder.unwrap() {
                if file.is_err() {
                    continue;
                }

                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                let file_path = file.path().into_os_string().into_string().unwrap();

                if file_name.ends_with(".bzx") {
                    continue;
                }

                println!("Running {}", file_path);

                let cmd = Command::new(format!("{}", file_path))
                    .output()
                    .expect(&format!("failed to execute {}", file_name));
                assert!(cmd.status.success());
            }
        }
    }
}
