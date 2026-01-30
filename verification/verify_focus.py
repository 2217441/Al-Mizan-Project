from playwright.sync_api import sync_playwright, expect

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()

        # Navigate to the mock page
        page.goto("http://localhost:8000/verification.html")

        # Wait for fonts/styles to load a bit
        page.wait_for_timeout(1000)

        # Press Tab to focus the first link (our card)
        page.keyboard.press("Tab")

        # Locate the card
        card = page.locator("#test-card")

        # Check if focused
        expect(card).to_be_focused()

        # Take a screenshot
        page.screenshot(path="verification/focus_result.png")

        print("Screenshot saved to verification/focus_result.png")
        browser.close()

if __name__ == "__main__":
    run()
