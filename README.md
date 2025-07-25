# rust_abstract_on_doc


Дока 4/ Krukowski 3

Список терминов:
//Stack - int, char, bool
//Heap - String, HashMap, Vec...

Ownership - владение
borrowing - заимствование (String -> &String)
scope
копирование/пермещение (по умолчанию простые типы копируются, а Heap-типы перемещаются (указателем))

Дока 5/ Krukowski 5

#### Structs
```rust
#[derive(Debug)] //need to write in console
struct Car {
    wheels: i32,
}
println!("{:#?}", my_car); // -> struct in console
```

#### enum -> String

```rust
use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
```