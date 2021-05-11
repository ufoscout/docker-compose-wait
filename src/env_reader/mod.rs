use std::env;

pub fn env_var(key: &str, default: String) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_e) => default,
    }
}

pub fn env_var_exists(key: &str) -> bool {
    env::var(key).is_ok()
}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    #[test]
    fn should_return_an_env_variable() {
        let mut env_key = String::from("");
        let mut env_value = String::from("");

        for (key, value) in env::vars() {
            // println!("Variable found [{}]: [{}]", key, value);
            if !value.trim().is_empty() {
                env_key = key;
                env_value = value;
            }
        }

        println!("Result Variable [{}]: [{}]", env_key, env_value);

        assert!(env_var_exists(&env_key));
        assert_ne!(env_value, String::from(""));
        assert_eq!(env_value, env_var(&env_key, String::from("")));
    }

    #[test]
    fn should_return_the_default_value_if_env_variable_not_present() {
        let mut random: i64 = rand::random();
        let env_key = random.to_string();
        random = random + 10;

        assert!(!env_var_exists(&env_key));
        assert_eq!(random.to_string(), env_var(&env_key, random.to_string()));
    }
}
