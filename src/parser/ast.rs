

#[derive(Clone, Debug, PartialEq, Eq)]
struct Identifier {
	name: String
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Block {
	expressions: Vec<Expression>
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Expression {
	identifiers: Vec<Identifier>,
	blocks: Vec<Block>
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
	Identifier(Identifier),
	Block(Block),
	Expression(Expression)
}
