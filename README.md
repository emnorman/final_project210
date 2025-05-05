# final_project210

My final DS210 project taht analyzes the structure of a real-world social graph (Facebook) using graph algorithms like degree distribution, shortest path (BFS), and graph diameter.

## Project Goal

This project answers the question:

> **"How connected is Facebook, and how we understand its structure?"**

Using the **Facebook Combined Ego Network dataset** from Stanford SNAP, I analyzed:
- Degree distribution (who's most connected?)
- Shortest path between users (how closely connected people are)
- Overall graph diameter (longest shortest path - how many steps it would take any user to reach another user) 

---

## Dataset

- `data/facebook_combined.txt`
- Source: [SNAP Facebook Dataset](https://snap.stanford.edu/data/ego-Facebook.html)
- 4,039 nodes (users) and 88,234 undirected edges (friendships)

---

## How to Run

Install: [Rust installed](https://www.rust-lang.org/tools/install).

```bash
# Clone the repo
git clone https://github.com/YOUR_USERNAME/final_project210.git
cd final_project210

# Build and run
cargo run
