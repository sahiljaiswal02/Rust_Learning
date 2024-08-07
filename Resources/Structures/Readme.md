# Using Structs and Methods to Structure Related Data
### Definition of Structs: 
> A struct (short for "structure") is a custom data type in Rust that groups together multiple related values under one name. Each value within a struct is called a field and can have different types.

### Comparison with Tuples:
> Unlike tuples, which group a fixed number of elements with potentially different types, structs provide named fields. This makes structs more readable and self-descriptive compared to tuples, where you access values by position rather than by name.

### Defining Structs:
> Structs are defined using the struct keyword followed by the name of the struct and a block containing its fields. Each field is declared with a name and a type, which enhances code clarity and maintenance.

### Instantiating Structs:
> You create an instance of a struct by specifying values for its fields. This can be done using a syntax similar to field names in the definition. Struct instances can be mutable or immutable depending on the declaration.

### Methods:
> Structs can have methods (functions that are associated with a particular struct type) and other associated functions. Methods are defined within an impl block and can access and modify the struct's fields.

### Associated Functions:
> Associated functions are defined with impl but do not take self as a parameter and are typically used for functions that do not operate on instance data.

### Use of Structs in Rust:
> Structs are integral to Rust's type system, allowing you to create complex data types and ensuring type safety at compile time. They help in encapsulating related data and functionality, leading to more organized and manageable code.

> Structs are ideal for creating domain-specific data models, such as representing a user profile, a point in a coordinate system, or a configuration for an application. They allow you to bundle related data and operations into a single, cohesive unit.
