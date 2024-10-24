import requests
from bs4 import BeautifulSoup
from database import create_database, save_rewards_to_db
import time

# After your request
time.sleep(2)  # Delay for 2 seconds before the next request

# Create the database and tables
create_database()

# URL of the Chick-fil-A One Rewards page
url = 'https://www.chick-fil-a.com/one'

# Set custom headers
headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.5938.62 Safari/537.36',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
    'Accept-Language': 'en-US,en;q=0.5',
    'Connection': 'keep-alive',
    'Upgrade-Insecure-Requests': '1',
    'Cache-Control': 'max-age=0',
}


session = requests.Session()
response = session.get(url, headers=headers)

# If still getting a 403, try adding a cookie
cookie = {
    'cookie_name': 'cookie_value'  # Replace with actual cookie data
}
response = session.get(url, headers=headers, cookies=cookie)


# Check if the request was successful
if response.status_code == 200:
    print("Successfully retrieved the page!")

    # Parse the page content with Beautiful Soup
    soup = BeautifulSoup(response.text, 'html.parser')

    # Find the rewards items
    rewards_items = soup.select('div.blurb')  # Adjust the selector if necessary

    # List to store the rewards data
    rewards_data = []

    # Loop through the found items and extract the reward descriptions
    for item in rewards_items:
        reward_description = item.p.text.strip() if item.p else "No Description"
        rewards_data.append({'company_id': 1, 'name': 'Sample Reward', 'description': reward_description})  # Use dictionary

    # Print and save the extracted rewards data
    if rewards_data:
        print("Rewards:")
        for data in rewards_data:
            print(f"Reward Description: {data['description']}")
        # Save rewards data to the database
        save_rewards_to_db(rewards_data)
    else:
        print("No rewards data found.")
else:
    print(f"Failed to retrieve the page. Status code: {response.status_code}")
    print(response.text)  # Print the response text for debugging
