pub mod day_x
{
    use std::fs;
    use std::path::Path;

    #[cfg(test)]
    mod tests {
        // use super::*;

    }

    fn _read_file(filename: &str) -> String
    {
        let file_path = Path::new(filename);

        if file_path.exists() == false
        {
            panic!("File does not exist");
        }

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        contents
    }

    pub fn _do_task_1()
    {

    }
}