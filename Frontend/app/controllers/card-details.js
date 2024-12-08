import Controller from '@ember/controller';
import { action } from '@ember/object';
import { tracked } from '@glimmer/tracking';

export default class CardDetailsController extends Controller {
  @tracked rating = 0; // Store the rating state
  @tracked card = {}; // Store the current card details

  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || 'Guest';
    sessionStorage.setItem('username', this.username);
    this.loadCardDetails(); // Fetch card details when the controller is initialized
  }

  // Fetch card details using the model's cardId
  async loadCardDetails() {
    try {
      const cardId = this.model.cardId;
      const response = await fetch(`http://localhost:8080/cards/${cardId}`);

      if (response.ok) {
        const cardData = await response.json();
        this.card = cardData;
        this.rating = cardData.rating || 0; // Set the initial rating from the card data
      } else {
        console.error('Failed to fetch card details.');
      }
    } catch (error) {
      console.error('Network error while fetching card details:', error);
    }
  }

  // Update the card's rating
  @action
  async setRating(newRating) {
    this.rating = newRating;

    try {
      const cardId = this.model.cardId;
      const response = await fetch(`http://localhost:8080/cards/${cardId}/rating`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ rating: this.rating }),
      });

      if (response.ok) {
        console.log(`Successfully updated card rating to ${this.rating} stars.`);
      } else {
        console.error('Failed to update card rating.');
      }
    } catch (error) {
      console.error('Network error while updating card rating:', error);
    }
  }
}
