import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class WalletController extends Controller {
  // Initialize username from sessionStorage or default to an empty string
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || '';
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  // Action to handle adding a card
  @action
  async submitCard(event) {
    event.preventDefault();
  
    const formData = new FormData(event.target);
    const cardData = {
      name: formData.get('cardName') || 'Default Name', // Default name
      type: formData.get('cardType') || 'Credit', // Ensure `type` is a valid card type
      icon: formData.get('iconChoice') || 'fa-credit-card', // Ensure icon matches a valid choice
      color: formData.get('cardColor') || '#000000', // Default color
      created: new Date().toISOString(),
      updated: new Date().toISOString(),
      company_id: 1, // Replace with dynamic company ID if needed
    };
  
    try {
      const response = await fetch('http://localhost:8080/add_card', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(cardData),
      });
  
      if (response.ok) {
        alert('Card added successfully!');
        await this.updateCards(); // Refresh the card list
      } else {
        const error = await response.text();
        alert(`Error adding card: ${error}`);
      }
    } catch (error) {
      console.error('Network error while adding card:', error);
      alert('An error occurred while adding the card.');
    }
  }
  
  // Action to handle deleting a card
  @action
  async deleteCard(cardId) {
    try {
      const response = await fetch(`http://localhost:8080/delete_card/${cardId}`, {
        method: 'DELETE',
      });

      if (response.ok) {
        alert('Card deleted successfully!');
        await this.updateCards(); // Refresh the card list
      } else {
        const error = await response.text(); // Read error response
        alert(`Error deleting card: ${error}`); // Display the error
      }
    } catch (error) {
      console.error('Network error while deleting card:', error); // Log network error
      alert('An error occurred while deleting the card.');
    }
  }

  // Fetch the latest cards and update the model
  async updateCards() {
    try {
      const response = await fetch('http://localhost:8080/cards');
      if (response.ok) {
        const updatedCards = await response.json();
        console.log('Updated card list:', updatedCards);
        updatedCards.forEach((card, index) => {
          console.log(`Card ${index}:`, card);
        });
        this.set('model', updatedCards);
      } else {
        console.error('Failed to fetch cards.');
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  }  
}
