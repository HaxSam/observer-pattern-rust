use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait EventArgs {}

trait Observable<T: EventArgs> {
	fn subscripe(&mut self, observer: Rc<RefCell<dyn Observer<T>>>) -> Weak<RefCell<dyn Observer<T>>>;
	fn unsubscribe(&mut self, observer: Weak<RefCell<dyn Observer<T>>>);
	fn raise(&mut self, args: T);
}

trait Observer<T: EventArgs> {
	fn get_rc(self) -> Rc<RefCell<dyn Observer<T>>>;
	fn update(&mut self, args: &T);
}

struct TestEventArgs {
	value: i32,
}
impl EventArgs for TestEventArgs {}

#[derive(Clone)]
struct TestObservable {
	observers: Vec<Weak<RefCell<dyn Observer<TestEventArgs>>>>,
}

impl TestObservable {
	fn new() -> Self {
		TestObservable { observers: Vec::new() }
	}
}

impl Observable<TestEventArgs> for TestObservable {
	fn subscripe(&mut self, observer: Rc<RefCell<dyn Observer<TestEventArgs>>>) -> Weak<RefCell<dyn Observer<TestEventArgs>>> {
		let observer_weak = Rc::downgrade(&observer);
		self.observers.push(observer_weak.clone());
		observer_weak
	}

	fn unsubscribe(&mut self, observer: Weak<RefCell<dyn Observer<TestEventArgs>>>) {
		self.observers.retain(|o| !o.ptr_eq(&observer));
	}

	fn raise(&mut self, args: TestEventArgs) {
		for observer in self.observers.clone() {
			match observer.upgrade() {
				Some(o) => {
					if Rc::weak_count(&o) == 2 {
						self.unsubscribe(observer.clone());
						continue;
					}
					o.borrow_mut().update(&args);
				}
				None => self.unsubscribe(observer.clone()),
			}
		}
	}
}

struct TestObserver {
	name: String,
	value: i32,
}

impl TestObserver {
	fn new(name: String) -> Self {
		TestObserver { name, value: 0 }
	}
}

impl Observer<TestEventArgs> for TestObserver {
	fn get_rc(self) -> Rc<RefCell<dyn Observer<TestEventArgs>>> {
		Rc::new(RefCell::new(self))
	}
	fn update(&mut self, args: &TestEventArgs) {
		self.value += args.value;
		println!("{} received {}", self.name, self.value);
	}
}

fn main() {
	let mut observable = TestObservable::new();

	let observer1 = TestObserver::new("Observer 1".to_string()).get_rc();
	let observer2 = TestObserver::new("Observer 2".to_string()).get_rc();
	let observer3 = TestObserver::new("Observer 3".to_string()).get_rc();

	let _o1 = observable.subscripe(observer1.clone());
	let _o2 = observable.subscripe(observer2.clone());
	let _o3 = observable.subscripe(observer3.clone());

	observable.raise(TestEventArgs { value: 1 });
}
