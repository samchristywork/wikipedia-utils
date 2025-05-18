import wikipedia
import sys

def usage():
    print("Usage: python main.py <command>")
    print()
    print("Commands:")
    print("  random          Get a random Wikipedia page")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        usage()
        sys.exit(1)

    command = sys.argv[1]

    if command == "random":
        page = wikipedia.random()
        print(page)

    else:
        print(f"Unknown command: {command}")
        usage()
