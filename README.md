# Mcq TUI App

A lightweight terminal-based multiple-choice question (MCQ) revision tool built with [Ratatui](https://github.com/ratatui-org/ratatui).  
It provides a simple text-based interface for practicing MCQs directly in your terminal.

---

## Usage

Clone the repository and build the project in release mode:

```bash
cargo build --release
```

Run the app by supplying a `.txt` file containing your questions:

```bash
mcq [file.txt]
```

---

## Input File Format

The app expects a text file with questions and options structured like this:

```txt
[Question 1]#[Option 1]#[Option 2]#[Option 3]#[Option 4]|[Question 2]#[Option 1]#[Option 2]#[Option 3]#[Option 4]
```

- Each **question** is separated by `|`  
- Each **option** is separated by `#`  
- Exactly **4 options per question** are required

> [!CAUTION]  
> The app assumes every question has **exactly 4 options**. Supplying fewer or more will cause errors.

---
## Example

Example `questions.txt`:

```txt
What is the capital of France?#Paris#London#Berlin#Rome|2 + 2 equals?#3#4#5#6
```

Run:

```bash
mcq questions.txt
```

---

## Features
- Terminal UI powered by Ratatui
- Quick MCQ revision workflow
- Lightweight and dependency-free input format
- Optimized for speed and minimal resource usage

---

## License
MIT © 2025 Bharathesh Jaishankar

---
