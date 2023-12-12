with open("input10.txt") as f:
    input = f.read()


def readable_graph(inp):
    mapping = {
        "|": "║",
        "-": "═",
        "L": "╚",
        "J": "╝",
        "7": "╗",
        "F": "╔",
        ".": " ",
        "S": "░",
    }
    for old, new in mapping.items():
        inp = inp.replace(old, new)
    return inp


input = readable_graph(input)

with open("maze.txt", "w") as f:
    f.write(input)
