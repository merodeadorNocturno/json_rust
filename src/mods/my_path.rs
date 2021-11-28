use std::path::PathBuf;

pub fn get_dir_path(file_name: String, ext: String) -> PathBuf {
  let option_home_dir = dirs::home_dir();
  match option_home_dir {
    Some(mut dir) => {
      dir.push("Development");
      dir.push("rust");
      dir.push("read_json_from_file");
      dir.push("json_files");
      dir.push("json_files");
      dir.set_file_name(file_name);
      dir.set_extension(ext);

      dir
    },
    None => {
      let mut path: PathBuf = [
        r"/",
        "Users",
        "pills",
        "Development",
        "rust",
        "read_json_from_file"
      ].iter().collect();
      path.push("json_files");
      path.set_file_name("currencies");
      path.set_extension("json");

      path
    }
  }
}