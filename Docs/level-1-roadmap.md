# Level 1 — Physics Principles Behind the Simulation

*(with roadmap steps clearly annotated and explained in exhaustive detail)*

---

## Level 1 Roadmap

1. Add constant gravity (Y-axis)
2. Create a particle with position & velocity
3. Update position over time (Euler integration)
4. Detect floor collision & make it bounce
5. Spawn particles with random velocities
6. Add left & right wall collisions

We will explain each step thoroughly now.

---

## Step 1 — Add constant gravity (Y-axis)

### What's happening?

From physics: Newton's second law:
**F = m × a**

For an object of mass *m*, falling under gravity:
**F₉ = m × g**

Divide both sides by *m*, we get:
**a = g**

So regardless of the mass, the particle accelerates downwards at rate `g`.

### Why `g ≠ 9.8` ?

In real-world physics:

* `g = 9.81 m/s²`
* But our screen's vertical size is maybe **600 pixels**.
* If we use `g = 9.8 px/s²` → the particle only moves \~4.9 px in the first second:
  *y = 0.5 × g × t² ≈ 4.9*

That's too small to notice!

So we scale `g` to something appropriate for the pixel space & frame rate. For example:

* `g ≈ 500 px/s²`

Now after 1 second:
*y = 0.5 × g × t² = 250 px*

Much more visible and fits nicely into a 600px window.

#### In short:

> Gravity pulls everything down. On Earth it’s \~9.8 m/s² — too small for a screen, so we scale it up, e.g., `500 px/s²`.

### Code of Gravity

```cpp
const float g = 500.0f; // gravity in px/s²
```

---

## Step 2 — Create a particle with position & velocity

### Why track position & velocity?

To simulate motion we need to know:

* Where the particle **is** → position: `(x, y)` (in pixels)
* How fast & in what direction it's moving → velocity: `(vx, vy)` (in pixels/second)

Initial conditions:

* `x, y` → where we spawn the particle.
* `vx, vy` → maybe start at 0 or some random values (see Step 5).

We represent the particle as an **object** with these properties.

### In short

> We need to know **where the particle is** and **how it’s moving**, so we define:
> `x, y` = position
> `vx, vy` = velocity
> `radius` = size

### Code

```cpp
class Particle {
public:
    float x, y;      // position
    float vx, vy;    // velocity
    float radius;

    Particle(float startX, float startY, float r)
        : x(startX), y(startY), vx(0), vy(0), radius(r) {}
};
```

---

## Step 3 — Update position over time (Euler integration)

### Why does motion happen?

From kinematics:

* Velocity: `v = dx/dt` → change in position.
* Acceleration: `a = dv/dt` → change in velocity.

For discrete time steps `dt`, we approximate:

```
v_new = v_old + a × dt
x_new = x_old + v_new × dt
```

In our simulation:

* At every frame (say, 60 FPS → `dt ≈ 1/60 ≈ 0.0167` seconds), we update:

```
vy = vy + g × dt
y  = y + vy × dt
x  = x + vx × dt
```

This method is called **Euler integration**, because it takes the current velocity and applies it linearly over the time step.

### In short

> At each frame:
>
> * Add gravity to `vy` (so it accelerates downwards).
> * Update position by moving it along its current `vx`, `vy`.

### Code

```cpp
void update(float dt) {
    vy += GRAVITY * dt;  // gravity increases vertical speed
    x  += vx * dt;       // move horizontally
    y  += vy * dt;       // move vertically
}
```

---

## Step 4 — Detect floor collision & make it bounce

### Why does the particle bounce?

When the particle's bottom goes below the floor:
`y + radius > floor`

We know the floor exerts an upward force (reaction), reversing the vertical velocity.

In our simulation:

* Correct position:
  `y = floor - radius`
  (so the particle rests on the floor)
* Reverse and dampen velocity:
  `vy = -vy × r`
  where `r` (restitution) is between 0 and 1:

  * `r = 1` → perfectly elastic (bounces back fully)
  * `r < 1` → loses some energy with each bounce

### In short

> If the particle goes below the floor:
>
> * Push it back up to sit on top of the floor.
> * Reverse and reduce `vy` (like a real bounce).

### Code

```cpp
void handleFloorCollision(float floorY, float restitution) {
    if (y + radius > floorY) {
        y = floorY - radius;       // place it back on floor
        vy = -vy * restitution;   // reverse and dampen vy
    }
}
```

---

## Step 5 — Spawn particles with random velocities

### Why random velocities?

If all particles start the same way, the simulation looks boring & artificial.

Instead:

* Pick a random horizontal speed:
  `vx ∈ [-v_max, +v_max]`
* Vertical speed can also start at 0 or a random upward kick.

### In short

> If all particles move the same way, it looks boring — random initial speeds make it lively.

### Code

```cpp
#include <cstdlib> // for rand

float randomFloat(float min, float max) {
    return min + static_cast<float>(rand()) / RAND_MAX * (max - min);
}

// usage:
Particle p(300, 0, 10);
p.vx = randomFloat(-100, 100);   // random horizontal speed
p.vy = randomFloat(-100, 0);     // optional upward kick
```

---

## Step 6 — Add left & right wall collisions

### Why do particles bounce off walls?

Similar to the floor: When the particle touches the left or right walls:

* `x - radius < 0` → left wall
* `x + radius > windowWidth` → right wall

We:

* Correct position to stay within the window.
* Reverse horizontal velocity: `vx = -vx`

### In short

> If the particle hits the walls:
>
> * Push it back inside.
> * Reverse `vx`.

### Code

```cpp
void handleWallCollision(float windowWidth) {
    if (x - radius < 0) {
        x = radius;
        vx = -vx;
    } 
    else if (x + radius > windowWidth) {
        x = windowWidth - radius;
        vx = -vx;
    }
}
```

---

## Full Table of Equations & Behavior

| Roadmap Step                     | Physics & Equations                                            | Particle Behavior                                     |
| -------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------- |
| **Step 1 - Gravity**             | `a_y = g`                                                      | Always accelerates downward                           |
| **Step 2 - Position & velocity** | Track `x, y, vx, vy`                                           | We know where & how fast it moves                     |
| **Step 3 - Update over time**    | `vy = vy + g × dt`<br>`y = y + vy × dt`<br>`x = x + vx × dt`   | Moves & accelerates frame by frame                    |
| **Step 4 - Floor bounce**        | If `y+radius > floor`:<br>`y = floor-radius`<br>`vy = -vy × r` | Hits floor & bounces back                             |
| **Step 5 - Random spawn**        | Random `vx`, `vy`                                              | Starts with random horizontal & maybe vertical motion |
| **Step 6 - Wall bounce**         | If `x ± radius` out of bounds:<br>`x` corrected, `vx = -vx`    | Hits walls & rebounds                                 |

---

## Analogy

Imagine throwing a bouncy ball in a box:

* You throw it with some speed → initial velocity.
* Gravity pulls it down → accelerates downward.
* It hits the floor → bounces up (loses energy if `r < 1`).
* It hits walls → bounces horizontally.

You just break time into tiny slices and calculate what happens at each tiny slice.

---

### Now you get it, so you need to check the full working code.
