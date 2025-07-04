# **entropic\_rust**

*A simple 2D physics simulation in Rust using Bevy.*

---

## 🚀 **Project Overview**

This is the Rust implementation of the **Entropic** project.
It simulates 2D particle motion with:

* Gravity
* Simple collision against world boundaries
* Real-time rendering using the Bevy engine.

---

## 🧱 **Current Features**

* Basic particle movement
* Gravity applied over time
* Single-ball collision with boundaries
* Frame-based motion updates

---

## 🔧 **How to Run**

Make sure you have Rust and Bevy setup, then run:

```bash
git clone https://github.com/XXNOUR/Entropic
git checkout Light-rust
cd entropic_rust
cargo run
```

---

## ⚙️ **Dependencies**

* `bevy` — game engine & rendering
* `rand` — for randomizing initial values

---

## 📈 **Next Steps**

* Add multiple particles
* Particle-particle collision
* Energy loss on collision
* Improved collision response

---

## 🛠️ **Project Structure**

```
entropic_rust/
├── src/main.rs    → simulation code
├── Cargo.toml     → Rust dependencies
└── .gitignore     → ignores target/ builds
```

---

### 💡 This is the Rust side of the Entropic project.

The C++/C# side lives in a different branch.

