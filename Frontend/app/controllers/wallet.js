import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class WalletController extends Controller {
  @action
  async submitCard(event) {
    event.preventDefault(); // Prevent form submission from refreshing the page

    // Collect the form data
    const formData = new FormData(event.target);
    const cardData = {
      name: formData.get("cardName"),  // Name of the card
      rtype: formData.get("iconChoice"),  // Get the card type from the icon choice select
      icon: formData.get("iconChoice"),  // Icon choice (from select)
      color: formData.get("cardColor"),  // Color (from input[type=color])
    };

    // Send the data to the backend
    const response = await fetch('/add_card', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',  // Ensure the data is JSON
      },
      body: JSON.stringify(cardData),  // Convert the data to JSON
    });

    if (response.ok) {
      alert('Card added successfully!');

      // Fetch updated list of cards after adding the new card
      const updatedResponse = await fetch('/cards');
      const updatedCards = await updatedResponse.json();
      
      // Update the model with the updated cards
      this.set('model', updatedCards);
    } else {
      alert('Error adding card.');
    }
  }
}
