pub type Scope = Vec<String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
	pub name: String,
	pub scope: Scope,
}
