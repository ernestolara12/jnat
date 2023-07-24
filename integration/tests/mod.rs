pub mod hello;
pub mod call_static_method;

#[derive(Debug)]
pub struct IntegrationTest {
  pub name: &'static str,
  pub lib: &'static str,
  pub java_class: &'static str,
  pub test_fn: fn(String) -> bool,
}

inventory::collect!(IntegrationTest);
