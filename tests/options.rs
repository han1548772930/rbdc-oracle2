use rbdc::db::ConnectOptions;
use rbdc_oracle::OracleConnectOptions;

#[test]
fn test_set_uri() {
    let mut options = OracleConnectOptions::default();
    options
        .set_uri("oracle://scott:tiger@localhost:1521/XE")
        .expect("set uri");

    assert_eq!(options.username, "scott");
    assert_eq!(options.password, "tiger");
    assert_eq!(options.connect_string, "//localhost:1521/XE");
}

#[test]
fn test_default_builder() {
    let options = OracleConnectOptions::new()
        .username("user")
        .password("pwd")
        .connect_string("//db/service");

    assert_eq!(options.username, "user");
    assert_eq!(options.password, "pwd");
    assert_eq!(options.connect_string, "//db/service");
}
