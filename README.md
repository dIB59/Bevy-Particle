# Bevy Game Project

This is a simple game/app built with [Bevy](https://bevyengine.org/), a modern, data-driven game engine written in Rust. This guide will help **absolute beginners** set up and run the project on **Windows**, **macOS**, and **Linux**.

---
## 🎥 Demo Video

https://github.com/user-attachments/assets/e82d1f70-7a8a-4521-93f5-72d43cfcb25f

## Prerequisites

### 1. **Install Rust**

Rust includes `cargo`, the package manager used to build and run the


 project.

Run the following command in your terminal (PowerShell, Terminal, or Bash):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation:

* On **Windows**, restart your Command Prompt or PowerShell.
* On **macOS/Linux**, restart your terminal or run `source $HOME/.cargo/env`.

Then verify installation:

```bash
rustc --version
cargo --version
```

### 2. **Install a Code Editor (Optional)**

We recommend [Visual Studio Code](https://code.visualstudio.com/) with the "Rust Analyzer" extension.

---

## ⚙️ OS-Specific Setup Tips

### 🪟 Windows

* Use **PowerShell** or **Windows Terminal**.
* Make sure [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) are installed.
* Vulkan support is recommended for best Bevy performance. You may need to install the latest graphics drivers or [Vulkan SDK](https://vulkan.lunarg.com/sdk/home).

### 🍎 macOS

* Ensure you're using **macOS 10.15+ (Catalina or newer)**.
* You may need to install development tools:

  ```bash
  xcode-select --install
  ```

### 🐧 Linux

* Bevy requires **Vulkan**. Install it via your distro's package manager:

  **Ubuntu/Debian:**

  ```bash
  sudo apt update
  sudo apt install libvulkan1 vulkan-utils
  ```

  **Fedora:**

  ```bash
  sudo dnf install vulkan vulkan-tools
  ```

  **Arch:**

  ```bash
  sudo pacman -S vulkan-tools
  ```

* Also install common build tools:

  ```bash
  sudo apt install build-essential pkg-config libx11-dev libxi-dev libgl1-mesa-dev
  ```

---

## 📦 Setting Up the Project

```bash
git clone https://github.com/dIB59/Bevy-Particle.git
cd bevy-project
```

---

## ▶️ Running the Project

To build and run

```bash
cargo run
```

The first run may take several minutes as it compiles dependencies.

---

## 🐛 Troubleshooting

| Problem                    | Solution                                                                                                           |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `command not found: cargo` | Rust isn't installed or your terminal wasn't restarted. Run `source $HOME/.cargo/env` or restart.                  |
| Vulkan not found           | Make sure Vulkan drivers are installed for your OS (see above).                                                    |
| Crashes or black window    | Update your GPU drivers. Try switching to `bevy_winit::WinitSettings::new().with_vsync(true)` for low-end systems. |
| Build errors on Windows    | Install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).               |

---

## 📚 Resources

* [📖 Bevy Book](https://bevyengine.org/learn/book/introduction/)
* [📘 Rust Book](https://doc.rust-lang.org/book/)
* [🧠 Bevy Cheat Book](https://bevy-cheatbook.github.io/)
* [🔧 Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
