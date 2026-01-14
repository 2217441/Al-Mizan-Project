import re

def merge_templates():
    with open('almizan-core/templates/base.html', 'r') as f:
        base = f.read()

    with open('almizan-core/templates/graph.html', 'r') as f:
        graph = f.read()

    # Extract content from graph.html
    # Look for {% block content %}...{% endblock %}
    # But graph.html extends base.html.

    match = re.search(r'{% block content %}(.*?){% endblock %}', graph, re.DOTALL)
    if not match:
        print("Could not find block content in graph.html")
        return

    content = match.group(1)

    # Replace block content in base.html
    merged = re.sub(r'{% block content %}.*?{% endblock %}', content, base, flags=re.DOTALL)

    # Replace title
    merged = re.sub(r'{% block title %}.*?{% endblock %}', 'Graph Explorer', merged)

    # Replace /static/ with static/
    merged = merged.replace('/static/', 'static/')

    # Save to verification/mock_graph.html
    with open('verification/mock_graph.html', 'w') as f:
        f.write(merged)

    print("Created verification/mock_graph.html")

if __name__ == "__main__":
    merge_templates()
