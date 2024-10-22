import requests
from bs4 import BeautifulSoup
from database import create_database, save_rewards_to_db  # Import functions from database.py

# URL of the Chick-fil-A One Rewards page
url = 'https://www.chick-fil-a.com/one'

# Set custom headers
headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
    'Accept-Language': 'en-US,en;q=0.5',
}

# Send a request to fetch the page content with headers
response = requests.get(url, headers=headers)

# Check if the request was successful
if response.status_code == 200:
    print("Successfully retrieved the page!")

    # Create the database and tables
    create_database()

    # Parse the page content with Beautiful Soup
    soup = BeautifulSoup(response.text, 'html.parser')

    # Find the rewards items
    rewards_items = soup.find_all('li')

    # List to store the rewards data
    rewards_data = []

    # Loop through the found items and extract the reward descriptions
    for item in rewards_items:
        description = item.find('div', class_='blurb')
        if description:
            reward_description = description.p.text.strip()
            rewards_data.append({'company_id': 1, 'name': 'Sample Reward', 'description': reward_description})

    # Print and save the extracted rewards data
    print("Rewards:")
    for data in rewards_data:
        print(f"Reward Description: {data['description']}")

    # Save rewards data to the database
    save_rewards_to_db(rewards_data)

else:
    print(f"Failed to retrieve the page. Status code: {response.status_code}")
