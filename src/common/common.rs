pub mod common{
    use std::{fs};
    use std::collections::HashMap;
    use std::fs::{File};
    use std::io::{Read};
    use serde::{Deserialize,Serialize};
    use std::path::{Path, PathBuf};

    const CONFIG_DIR_NAME: &'static str = "config";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub app_id: String,
        pub get_host_uri: String,
        pub cam_id: String,
        pub local_addr: String,
        pub server_addr: String,
        pub cidr: String,
        pub key: String,
        pub obfuscate: bool,
    }

    impl Config{
        pub fn init(self){
            let json_config = serde_json::to_string(&self).unwrap();
            println!("{}",json_config);
        }

        pub async fn get_cidr(&mut self)  {
            let mut body = HashMap::new();
            body.insert("objectId",&self.cam_id);
            let client = reqwest::Client::builder()
                .build().expect("Error build client");
            let  resp = client.post(self.get_host_uri.as_str())
                .json(&body)
                .header("Content-Type","application/json")
                .header("application/json",self.app_id.as_str())
                .send()
                .await
                .expect("Error response");
            if resp.content_length().unwrap() > 0 && resp.status().is_server_error()  {
                let t = resp.text().await.unwrap();
                self.cidr = t + "/16";
            }
        }
    }

    pub fn get_ipv4() -> (String,String){
        return (String::from("b"), String::from("a"))
    }

    pub async fn get_config() -> Config  {
        let mut data = String::new();
        File::open(get_config_file_name())
            .unwrap()
            .read_to_string(&mut data)
            .expect("Error read files json");
        let f:serde_json::Value = serde_json::from_str(data.as_str()).unwrap();
        let b = Config::deserialize(f).expect("Error deserialize");
        return b
    }

    pub fn check_path(ex_path: &Path) {
        let config_dir_path = Path::join(ex_path, CONFIG_DIR_NAME);
        if config_dir_path.exists() != true {
            fs::create_dir_all(config_dir_path).unwrap();
        }
    }

    pub fn get_config_file_name() -> PathBuf {
        let ex = Path::new("./");
        check_path(ex);
        let us =  Path::join(CONFIG_DIR_NAME.as_ref(), "config.json");
        us
    }
}