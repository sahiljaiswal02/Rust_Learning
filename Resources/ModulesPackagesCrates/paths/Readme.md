# Paths for Referring to an Item in the Module Tree
> Path can take two forms:
- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.

#### Example: 
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

## Starting Relative Paths with super
> `super` allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier
#### Example:
```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

## Bringing Paths into Scope with the use Keyword
>  `use` only creates the shortcut for the particular scope in which the use occurs
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
> By adding use `crate::front_of_house::hosting` in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.

## Providing New Names with the as Keyword
> with `use`: after the path, we can specify `as` and a new local name, or alias, for the type
#### Example:
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```
