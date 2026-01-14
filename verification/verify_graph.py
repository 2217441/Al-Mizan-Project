import os
from playwright.sync_api import sync_playwright

def verify_graph_search():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Load the mock HTML file
        file_path = os.path.abspath('verification/mock_graph.html')
        page.goto(f'file://{file_path}')

        # Wait for graph to load (simulated by timeout or waiting for elements)
        page.wait_for_timeout(2000) # Wait for network init and hotfix

        # Check initial state: Search input empty, clear button hidden
        clear_btn = page.locator('#search-clear-btn')
        search_input = page.locator('#search-input')

        if clear_btn.is_visible():
            print("Error: Clear button should be hidden initially")
        else:
            print("Success: Clear button hidden initially")

        # Type in search
        search_input.fill('Allah')
        search_input.dispatch_event('input') # Trigger oninput

        # Check if clear button is visible
        if clear_btn.is_visible():
            print("Success: Clear button visible after typing")
        else:
            print("Error: Clear button should be visible after typing")

        # Take screenshot 1: With search text
        page.screenshot(path='verification/graph_search_active.png')

        # Click clear button
        clear_btn.click()

        # Check if input is cleared and button hidden
        if search_input.input_value() == '':
            print("Success: Input cleared")
        else:
            print(f"Error: Input not cleared. Value: {search_input.input_value()}")

        if not clear_btn.is_visible():
            print("Success: Clear button hidden after clearing")
        else:
            print("Error: Clear button should be hidden after clearing")

        # Take screenshot 2: Cleared
        page.screenshot(path='verification/graph_search_cleared.png')

        browser.close()

if __name__ == "__main__":
    verify_graph_search()
