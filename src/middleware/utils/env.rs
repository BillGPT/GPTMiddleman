// 引入依赖库
use dotenv::dotenv;
use std::env;

fn load_env_variables() -> Vec<(String, String)> {
    // 载入.env文件
    dotenv().ok();

    // 将环境变量转换为一个 Vec<(String, String)> 类型的数组
    let variables: Vec<(String, String)> = env::vars().collect();

    variables
}

pub fn get_env_var(key: &str) -> String {
    let value = load_env_variables().iter().find_map(|(k, v)| {
        if k == key {
            Some(v.to_owned())
        } else {
            None
        }
    });
    value.unwrap_or_default()
}