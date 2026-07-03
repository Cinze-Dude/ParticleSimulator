import sys
import json
import turtle
from typing import Any

screen = turtle.Screen()
screen.tracer(0)
screen.setup(width=800, height=800)

t = turtle.Turtle()
t.hideturtle()
t.penup()

def draw_point(particle: Any, pos_factor: float = 10.0, radius_factor: float = 10.0) -> None:
    t.goto(particle["state"]["transform"]["position"]["x"] * pos_factor, particle["state"]["transform"]["position"]["y"] * pos_factor)
    t.color(particle["attr"]["color"]["r"] / 255, particle["attr"]["color"]["g"] / 255, particle["attr"]["color"]["b"] / 255)
    t.dot(particle["attr"]["radius"] * radius_factor)
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