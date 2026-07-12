import tkinter as tk
import math
import random

def f(x, y): return (2*x + 4*y)%5

def pos(a):
    r = (a/5) * 2.0*math.pi
    return 140 + math.sin(r) * 100, 200 - (60 + math.cos(r) * 100)

def draw(a, color, size, text=None):
    x,y = pos(a)
    canvas.create_oval(x-size, y-size, x+size, y+size, fill=color, outline="")
    if text:
        canvas.create_text(x, y, text=text, fill="black", font=("Arial", 20))

root = tk.Tk()
canvas = tk.Canvas(root, width=800, height=600)
canvas.pack()

# x = f(y, f(x, f(f(y, x), y)
moves = [
    lambda: f(y, x),
    lambda: f(v, y),
    lambda: f(x, v),
    lambda: f(y, v),
]

move_i = 0

def act(event):
    global x, y, v, move_i

    canvas.delete("all")

    for i in range(5):
        draw(i, "black", 30)


    if move_i == 0:
        x = random.randint(0, 4)
        y = random.randint(0, 4)

    draw(x, "red", 20, text="x")
    draw(y, "yellow", 20, text="y")

    if move_i != 0:
        mv = moves[move_i-1]

        v = mv()
        draw(v, "green", 10)

    t = """
        * = f(y, x)

        * = f(*, y)

        * = f(x, *)

        * = f(y, *)
    """
    canvas.create_text(370, 140, text=t, fill="white", font=("Courier", 20))

    if move_i != len(moves):
        canvas.create_oval(340, 35 + move_i * 57, 360, 55 + move_i * 57, fill="green", outline="")

    move_i = (move_i+1)%(len(moves)+1)

root.bind("<space>", act)
act(None)
root.mainloop()
