import wikipedia
import sys

def usage():
    print("Usage: python main.py <command>")
    print()
    print("Commands:")
    print("  search <query>  Search Wikipedia for a query")
    print("  summary <page>  Get a summary of a Wikipedia page")
    print("  random          Get a random Wikipedia page")
    print("  page <page>     Get the full text of a Wikipedia page")
    print("  sections <page> Get sections of a Wikipedia page")
    print("  section <page> <section> Get a specific section of a Wikipedia page")
    print("  links <page>    Get links from a Wikipedia page")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        usage()
        sys.exit(1)

    command = sys.argv[1]

    if command == "search":
        if len(sys.argv) < 3:
            print("Error: Missing query for search command")
            usage()
            sys.exit(1)
        query = " ".join(sys.argv[2:])
        results = wikipedia.search(query)
        for result in results:
            print(result)

    elif command == "summary":
        if len(sys.argv) < 3:
            print("Error: Missing page name for summary command")
            usage()
            sys.exit(1)
        page_name = " ".join(sys.argv[2:])
        try:
            summary = wikipedia.summary(page_name)
            print(summary)
        except wikipedia.exceptions.DisambiguationError as e:
            print(f"Disambiguation error: {e}")
        except wikipedia.exceptions.PageError as e:
            print(f"Page error: {e}")

    elif command == "random":
        page = wikipedia.random()
        print(page)

    elif command == "page":
        if len(sys.argv) < 3:
            print("Error: Missing page name for page command")
            usage()
            sys.exit(1)
        page_name = " ".join(sys.argv[2:])
        try:
            page = wikipedia.WikipediaPage(page_name)
            print(page.content)
        except wikipedia.exceptions.DisambiguationError as e:
            print(f"Disambiguation error: {e}")
        except wikipedia.exceptions.PageError as e:
            print(f"Page error: {e}")

    elif command == "section":
        if len(sys.argv) < 4:
            print("Error: Missing page name or section for section command")
            usage()
            sys.exit(1)
        page_name = " ".join(sys.argv[2:-1])
        section_name = sys.argv[-1]
        try:
            page = wikipedia.page(page_name)
            section_content = page.section(section_name)
            if section_content:
                print(section_content)
            else:
                print(f"Section '{section_name}' not found in page '{page_name}'")
        except wikipedia.exceptions.DisambiguationError as e:
            print(f"Disambiguation error: {e}")
        except wikipedia.exceptions.PageError as e:
            print(f"Page error: {e}")

    elif command == "links":
        if len(sys.argv) < 3:
            print("Error: Missing page name for links command")
            usage()
            sys.exit(1)
        page_name = " ".join(sys.argv[2:])
        try:
            page = wikipedia.page(page_name)
            links = page.links
            for link in links:
                print(link)
        except wikipedia.exceptions.DisambiguationError as e:
            print(f"Disambiguation error: {e}")
        except wikipedia.exceptions.PageError as e:
            print(f"Page error: {e}")

    else:
        print(f"Unknown command: {command}")
        usage()
