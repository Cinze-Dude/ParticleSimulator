import sys
import json
import turtle
from typing import Any

screen = turtle.Screen()
screen.tracer(0)

t = turtle.Turtle()
t.hideturtle()
t.penup()

def draw_point(particle: Any):
    # A particle has apprarent states like these: position, color, size
    t.goto(particle["state"]["transform"]["position"]["x"] * 10, particle["state"]["transform"]["position"]["y"] * 10)
    t.color(particle["attr"]["color"]["r"] / 255, particle["attr"]["color"]["g"] / 255, particle["attr"]["color"]["b"] / 255)
    t.dot(particle["attr"]["radius"] * 10)
for line in sys.stdin:
    try:
        frame = json.loads(line)
    except:
        continue

    t.clear()

    for p in frame["particles"]:
        draw_point(p)

    screen.update()

screen.mainloop()