use oop::extend;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bar(i32);

impl Bar {
	pub fn new(x: i32) -> Self {
		Self(x)
	}
	
	pub fn baz(&self) {
		println!("Nya~, our number is {}", self.0);
	}
}

#[extend(Bar, 0)]
struct Foo(Bar);

#[test]
pub fn test2() {
	let bar = Bar::new(2);
	let foo = Foo(bar.clone());
	foo.baz();
	assert_eq!(bar, foo._super().clone());
	println!("foo._super() == {:?}", foo._super());
}
