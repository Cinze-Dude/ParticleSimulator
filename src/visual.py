import sys
import json
import turtle

screen = turtle.Screen()
screen.tracer(0)

t = turtle.Turtle()
t.hideturtle()
t.penup()

for line in sys.stdin:
    try:
        frame = json.loads(line)
    except:
        continue

    t.clear()

    for p in frame["particles"]:
        x, y, z = p["pos"]
        t.goto(x * 100, y * 100)
        print(p["pos"][x])
        t.dot(10)

    screen.update()

screen.mainloop()