# Understanding Object-Oriented Programming in C++ — With Explained Code

This document explains why we use OOP for the 2D physics simulation project, reviews key OOP concepts, explains how OOP works in C++, and provides a detailed explanation of the standard Animal/Cat/Dog example in Java and C++.

---

## 1. Why Use OOP in This Project?

While you *could* write the simulation procedurally, OOP makes the codebase more manageable and extensible as complexity grows.

For example:

* **Modularity** — Encapsulate data & behavior for particles, forces, collisions.
* **Reusability** — The `Particle` class could also be used in a different simulation.
* **Maintainability** — Fewer global variables, cleaner separation of logic.
* **Domain modeling** — Physics concepts (particles, forces) are naturally represented as objects.

It is not mandatory — we simply choose it because it pays off later.

---

## 2. Quick Overview of OOP Concepts

* **Encapsulation** — Data + behavior packaged inside a class.
* **Inheritance** — A new class derives from another and inherits its methods and attributes.
* **Polymorphism** — Base class pointers or references can call overridden methods of derived classes at runtime.
* **Abstraction** — Users interact with a clean interface without worrying about implementation.
* **Access control** — `public`, `protected`, `private` determine visibility.
* **Constructors/Destructors** — Special functions for creating & destroying objects.

---

## 3. How OOP Works in C++

### Header Files vs. Source Files

* `.hpp` (header) — Declares the structure of a class — its members and method signatures — but not the method bodies.
* `.cpp` (source) — Implements the functionality declared in the header.
* Header guards (`#ifndef … #define … #endif`) prevent duplicate declarations.

### Compilation

* Each `.cpp` file is compiled into an object file.
* The linker puts these object files together into a single executable.

### Virtual Functions & Polymorphism

* Use `virtual` in base classes to allow derived classes to override methods.
* Use `override` in derived classes to ensure you are actually overriding.
* Always define a **virtual destructor** when working with inheritance to clean up correctly.

```cpp
class Animal {
public:
    virtual void speak();
    virtual ~Animal();
};
```

### Memory Management

* Create on stack: `Animal a;` — automatically destroyed.
* Create on heap: `Animal* a = new Animal();` — you must `delete a;` later.
* Modern C++ favors smart pointers: `std::unique_ptr`, `std::shared_ptr`.

### Common Confusions

* Header guards are critical.
* Forgetting to link `.cpp` files results in linker errors.
* Object slicing happens if you assign a derived object to a base-class *value*, losing derived parts.

---

## 4. Animal/Cat/Dog Example: Explained

We’ll build the same scenario in Java and C++, then explain it line by line.

---

### Java Example

```java
// Animal.java
public class Animal {
    public void speak() {
        System.out.println("Some animal sound");
    }
}

// Cat.java
public class Cat extends Animal {
    @Override
    public void speak() {
        System.out.println("Meow");
    }
}

// Dog.java
public class Dog extends Animal {
    @Override
    public void speak() {
        System.out.println("Woof");
    }
}

// Main.java
public class Main {
    public static void main(String[] args) {
        Animal a1 = new Cat();
        Animal a2 = new Dog();
        a1.speak();  // prints "Meow"
        a2.speak();  // prints "Woof"
    }
}
```

#### Explanation

* `Animal` declares `speak()`, `Cat` and `Dog` override it.
* You can treat a `Cat` or `Dog` as an `Animal` because of inheritance.
* Calling `speak()` calls the correct implementation thanks to polymorphism.

---

### C++ Example

#### Animal.hpp

```cpp
#ifndef ANIMAL_HPP
#define ANIMAL_HPP

class Animal {
public:
    virtual void speak();
    virtual ~Animal() = default;
};

#endif
```

#### Animal.cpp

```cpp
#include <iostream>
#include "Animal.hpp"

void Animal::speak() {
    std::cout << "Some animal sound" << std::endl;
}
```

#### Cat.hpp

```cpp
#ifndef CAT_HPP
#define CAT_HPP

#include "Animal.hpp"

class Cat : public Animal {
public:
    void speak() override;
};

#endif
```

#### Cat.cpp

```cpp
#include <iostream>
#include "Cat.hpp"

void Cat::speak() {
    std::cout << "Meow" << std::endl;
}
```

#### Dog.hpp

```cpp
#ifndef DOG_HPP
#define DOG_HPP

#include "Animal.hpp"

class Dog : public Animal {
public:
    void speak() override;
};

#endif
```

#### Dog.cpp

```cpp
#include <iostream>
#include "Dog.hpp"

void Dog::speak() {
    std::cout << "Woof" << std::endl;
}
```

#### main.cpp

```cpp
#include "Animal.hpp"
#include "Cat.hpp"
#include "Dog.hpp"

int main() {
    Animal* a1 = new Cat();
    Animal* a2 = new Dog();

    a1->speak();  // prints "Meow"
    a2->speak();  // prints "Woof"

    delete a1;
    delete a2;
    return 0;
}
```

---

### Explanation

* `Animal` declares `speak()` as `virtual` so that derived classes can override it.
* `Cat` and `Dog` implement their own `speak()` methods.
* In `main.cpp`, we use pointers to `Animal` to hold `Cat` and `Dog` objects.
* When calling `speak()`, the correct implementation is chosen at runtime.
* We `delete` the pointers because we allocated them with `new`.

---

## Why This Matters for Our Simulation

* We can declare a base `Particle` class with common logic and extend it for specialized particles.
* OOP makes it easier to add new features without rewriting everything.
* Clear separation of interface (`.hpp`) and implementation (`.cpp`) improves readability.

