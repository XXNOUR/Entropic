
# How I’m Building My First 2D Physics Simulation (and How You Can Too)

## What are we building?

In this tutorial I’ll walk you through my own process of building a **2D physics simulation** using **C++ + SFML**.
If you follow along, you’ll learn:

* How to simulate particles moving under gravity
* How to handle collisions with walls & floor
* How to draw and animate everything in real-time
* How to set up your tools & environment step by step

If you’re curious about physics or just want to learn C++ and graphics programming, this is a great beginner-friendly project.

---

## My Roadmap

To keep things clear and achievable, I broke down the project into **levels**.
You can follow these same steps too.
On each point, I also mention the **equation** or **principle** being applied.

### Level 1: Core Motion

Here we build the simplest physics:

* Add constant gravity (Y-axis)
  Principle: constant downward acceleration.
  Equation: `vy = vy + g * dt`

* Create a particle with position & velocity
  Variables: `x, y, vx, vy`

* Update position over time (Euler integration)
  Equation: `position = position + velocity * dt`

* Detect floor collision & make it bounce
  Principle: reverse vertical velocity & apply damping.
  Equation: `vy = -vy * e` (where `e` is the restitution coefficient, < 1)

* Spawn particles with random velocities
  Initial condition: set random `vx, vy` on spawn.

* Add left & right wall collisions
  Principle: reflect horizontal velocity.
  Equation: `vx = -vx`

---

### Level 2: Better Physics

Once Level 1 works, we improve realism:

* Add friction & damping (slows particles down)
  Principle: velocity decreases over time or on collision.
  Equation: `velocity = velocity * f` (where `f < 1`)

* Add a restitution coefficient to control bounciness
  Already applied above (`e`).

* Let user spawn particles by clicking
  Just input handling.

* Add wind force
  Principle: horizontal constant force.
  Equation: `vx = vx + wind * dt`

* Make gravity adjustable at runtime
  No equation change — just make `g` a variable.

---

### Level 3 & Beyond

For advanced learners & polish:

* Particles attracting/repelling each other
  Newton’s law of gravitation or Coulomb’s law:
  `F = G * m1 * m2 / r^2` or `F = k * q1 * q2 / r^2`

* Add mass & calculate acceleration
  Newton’s second law:
  `a = F / m`

* Color effects, trails, UI, etc.
  Cosmetic — no physics.

---

## What tools do I need?

Here’s what I’m using — and what I recommend for you too.

### Compiler

We’ll write our code in C++ and compile it.
On Ubuntu, you probably already have `g++`.
Check:

```bash
g++ --version
```

On my machine:

```
g++ (Ubuntu 13.3.0) — perfect.
```

---

### Text Editor or IDE

Use what you like:

* VS Code (recommended — friendly, with extensions)
* Or terminal editors like `vim`, `nano`

---

### Graphics Library

For visuals, we’ll use:
**SFML — Simple and Fast Multimedia Library**

Why SFML?

* Beginner-friendly
* Great for 2D graphics
* Handles windows, shapes, events, etc.
* Works perfectly with C++

---

## Installing SFML on Ubuntu

### Step 1: Install library

```bash
sudo apt update
sudo apt install libsfml-dev
```

### Step 2: Verify installation

Check the headers:

```bash
ls /usr/include/SFML
```

You should see something like:

```
Audio.hpp  Graphics.hpp  Window.hpp …
```

---

## Writing Our First SFML Program

Let’s write a minimal SFML program to test everything works.
We’ll create a window and draw a green circle.

### Code

```cpp
#include <SFML/Graphics.hpp>

int main() {
    sf::RenderWindow window(sf::VideoMode(800, 600), "My Simulation");
    sf::CircleShape shape(50);
    shape.setFillColor(sf::Color::Green);

    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed)
                window.close();
        }

        window.clear();        // clear old frame
        window.draw(shape);    // draw circle
        window.display();      // show frame
    }

    return 0;
}
```

---

### How to Compile & Run

Compile:

```bash
g++ -std=c++17 main.cpp -o my-sim \
    -lsfml-graphics -lsfml-window -lsfml-system
```

Run:

```bash
./my-sim
```

You should see a window with a green circle.

---

## Explaining the Code

Here’s what happens step by step.

### Window Creation

```cpp
sf::RenderWindow window(sf::VideoMode(800, 600), "My Simulation");
```

Opens a window of size 800×600 with the title “My Simulation”.

### Shape Creation

```cpp
sf::CircleShape shape(50);
shape.setFillColor(sf::Color::Green);
```

Creates a circle of radius 50 pixels, colored green.

### Main Loop

```cpp
while (window.isOpen()) {
    …
}
```

Keeps running while the window is open.

### Event Handling

```cpp
sf::Event event;
while (window.pollEvent(event)) {
    if (event.type == sf::Event::Closed)
        window.close();
}
```

Checks if the user clicked the close button to close the window.

### Drawing

```cpp
window.clear();
window.draw(shape);
window.display();
```

Clears the screen → draws the circle → displays the frame.
This happens about 60 times per second.

---

## Why this is awesome

By the time you run this and understand it:

* You’ve set up your tools
* You’ve written and compiled a C++ graphics program
* You’re ready to start adding physics logic

---

## Why We Use OOP

For this project, we’ll use **Object-Oriented Programming (OOP)**.
Here’s why:

* Each particle has its own properties and behavior: position, velocity, radius, color, and how it updates & draws.
* OOP lets us bundle these into a single `Particle` class — making the code cleaner and easier to extend.
* When we add more particles or more complex interactions, it’s much more manageable to create and handle multiple `Particle` objects than managing many separate arrays or variables.
* This matches the real-world idea that each particle is a separate, self-contained thing.

You could write everything in a procedural style too, but OOP keeps the code organized and scalable as the project grows.
