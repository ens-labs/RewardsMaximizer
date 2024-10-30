# Rewards Maximizer Scraper

## Overview

The Rewards Maximizer Scraper is a Python-based web scraper that extracts reward data from various pages. It retrieves information about available rewards and stores it in a SQLite database for further analysis.

## Features

- Retrieves reward descriptions from the Chick-fil-A One, Navy Federal, and McDonald's rewards pages.
- Stores extracted reward data in a SQLite database.
- Utilizes Beautiful Soup for HTML parsing.
- Custom headers are used to simulate a web browser and avoid access restrictions.

## Requirements

- Python 3.x
- Libraries:
  - `requests`
  - `beautifulsoup4`
  - `sqlite3`

## Database Structure

The scraper creates and uses the following database tables:

    users
    companies
    rewards
    cards
    user_cards
    user_rewards
    user_feedback
    vendor_deals
    notifications
    comments
