use std::env;

pub fn env_var(key: &str, default: String) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_e) => default,
    }
}

#[cfg(test)]
mod test {

    extern crate time;

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

        assert_ne!(env_value, String::from(""));
        assert_eq!(env_value, env_var(&env_key, String::from("")));
    }

    #[test]
    fn should_return_the_default_value_if_env_variable_not_present() {
        let mut nanosec = time::get_time().nsec;
        let env_key = nanosec.to_string();
        nanosec = nanosec + 10;
        assert_eq!(nanosec.to_string(), env_var(&env_key, nanosec.to_string()));
    }

}
