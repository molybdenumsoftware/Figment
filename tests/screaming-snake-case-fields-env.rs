use figment::{providers::Env, Figment};

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct Config {
    FOO: String,
}

#[test]
fn empty_env_vars() {
    figment::Jail::expect_with(|jail| {
        jail.set_env("FOO", "bar");

        let config = Figment::new().merge(Env::raw()).extract::<Config>()?;

        assert_eq!(config.FOO, "bar");

        Ok(())
    });
}
